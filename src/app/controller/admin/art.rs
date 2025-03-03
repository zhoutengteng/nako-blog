use actix_web::{
    web, 
    Result, 
    Error, 
    HttpRequest,
    HttpResponse, 
};

use crate::nako::{
    time,
    utils,
    http as nako_http,
};
use crate::nako::global::{
    Session, 
    AppState,
    Validate,
    Serialize,
    Deserialize,
};

use crate::app::service::http;
use crate::app::entity::{
    self,
    art as art_entity,
    // cate as cate_entity,
};
use crate::app::model::{
    art,
    cate,
    user,
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/art/index.html", &ctx))
}

// ==========================

#[derive(Serialize)]
pub struct ListData {
    list: Vec<art_entity::Model>,
    count: u64,
}

#[derive(Deserialize)]
pub struct ListQuery {
    page: u64,
    limit: u64,

    uuid: Option<String>,
    title: Option<String>,
    is_top: Option<i32>,
    status: Option<i32>,
}

// 数据列表
pub async fn list(
    state: web::Data<AppState>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    let page: u64 = query.page;
    let per_page: u64 = query.limit;

    let search_where = art::ArtWhere{
        title: query.title.clone(),
        uuid: query.uuid.clone(),
        tag: None,
        cate_id: None,
        user_id: None,
        is_top: query.is_top,
        status: query.status,
    };
    let search_where = search_where.format();

    let (list, _num_pages) = art::ArtModel::search_in_page(
            db, 
            page, 
            per_page, 
            search_where.clone(),
        )
        .await.unwrap_or_default();
    let count = art::ArtModel::search_count(db, search_where.clone())
        .await.unwrap_or(0);

    let res: ListData = ListData{
        list: list,
        count: count,
    };

    Ok(nako_http::success_json("获取成功", res))
}

// ==========================

#[derive(Deserialize)]
pub struct DetailQuery {
    id: u32,
}

// 详情
pub async fn detail(
    state: web::Data<AppState>,
    query: web::Query<DetailQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    if query.id == 0 {
        return Ok(http::error_admin_html(&mut view, "ID不能为空", ""));
    }

    let data = art::ArtModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(http::error_admin_html(&mut view, "文章不存在", ""));
    }

    // 分类
    let cate_data = cate::CateModel::find_by_id(db, data.cate_id).await.unwrap_or_default().unwrap_or_default();

    // 作者
    let user_data = user::UserModel::find_user_by_id(db, data.user_id).await.unwrap_or_default().unwrap_or_default();

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &data);
    ctx.insert("cate", &cate_data);
    ctx.insert("user", &user_data);

    Ok(nako_http::view(&mut view, "admin/art/detail.html", &ctx))
}

// ==========================

// 添加
pub async fn create(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    let cate_list = cate::CateModel::find_all(db).await.unwrap_or_default();

    let mut ctx = nako_http::view_data();
    ctx.insert("cate_list", &cate_list);

    Ok(nako_http::view(&mut view, "admin/art/create.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct CreateForm {
    cate_id: u32,
    title: String,
    status: i32,
}

// 添加保存
pub async fn create_save(
    req: HttpRequest,
    session: Session, 
    state: web::Data<AppState>,
    params: web::Form<CreateForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if params.cate_id == 0 {
        return Ok(nako_http::error_json("所属分类不能为空"));
    }
    if params.title.as_str() == "" {
        return Ok(nako_http::error_json("文章不能为空"));
    }
    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_json("状态不能为空"));
    }

    let add_time = time::now().timestamp();

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(val) = req.peer_addr() {
        ip = val.ip().to_string();
    }

    let user_id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();

    let create_data = art::ArtModel::create(db, art_entity::Model{
            uuid:     utils::uuid(),
            cate_id:  params.cate_id,
            user_id:  user_id,
            title:    params.title.clone(),
            content:  "".to_string(),
            html_content: "".to_string(),
            views:    Some(0),
            status:   Some(params.status),
            add_time: Some(add_time),
            add_ip:   Some(ip.clone()),
            ..entity::default()
        }).await;
    if !create_data.is_ok() {
        return Ok(nako_http::error_json("添加失败"));
    }

    Ok(nako_http::success_json("添加成功", "")) 
}

// ==========================

#[derive(Deserialize)]
pub struct UpdateQuery {
    id: u32,
}

// 更新
pub async fn update(
    state: web::Data<AppState>,
    query: web::Query<UpdateQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    if query.id == 0 {
        return Ok(http::error_admin_html(&mut view, "ID不能为空", ""));
    }

    let info = art::ArtModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(http::error_admin_html(&mut view, "文章不存在", ""));
    }

    let cate_list = cate::CateModel::find_all(db).await.unwrap_or_default();

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &info);
    ctx.insert("cate_list", &cate_list);

    Ok(nako_http::view(&mut view, "admin/art/update.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateForm {
    cate_id: u32,
    cover: String,
    title: String,
    keywords: String,
    description: String,
    content: String,
    html_content: String,
    brief: String,
    tags: String,
    from: String,
    is_top: i32,
    status: i32,
    add_time: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UpdateValidate {
    #[validate(required(message = "所属分类不能为空"))]
    cate_id: Option<u32>,
    #[validate(required(message = "文章标题不能为空"))]
    title: Option<String>,
    #[validate(required(message = "内容不能为空"))]
    content: Option<String>,
    #[validate(required(message = "内容不能为空"))]
    html_content: Option<String>,
    #[validate(required(message = "简介不能为空"))]
    brief: Option<String>,
    #[validate(required(message = "状态不能为空"))]
    status: Option<i32>,
    #[validate(required(message = "发布时间不能为空"))]
    add_time: Option<String>,
}

// 更新保存
pub async fn update_save(
    state: web::Data<AppState>,
    query: web::Query<UpdateQuery>,
    params: web::Form<UpdateForm>,
) -> Result<HttpResponse, Error> {
    if query.id == 0 {
        return Ok(nako_http::error_json("ID不能为空"));
    }

    let vali_data = UpdateValidate{
        cate_id: Some(params.cate_id.clone()),
        title: Some(params.title.clone()),
        content: Some(params.content.clone()),
        html_content: Some(params.html_content.clone()),
        brief: Some(params.brief.clone()),
        status: Some(params.status.clone()),
        add_time: Some(params.add_time.clone()),
    };

    let vali = vali_data.validate();
    if vali.is_err() {
        return Ok(nako_http::error_json(format!("{}", vali.unwrap_err()).as_str()));
    }

    let db = &state.db;

    let info = art::ArtModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(nako_http::error_json("要更改的文章不存在"));
    }

    let add_time = time::parse(params.add_time.as_str()).timestamp();

    // 更新
    let data = art::ArtModel::update_by_id(db, query.id, art_entity::Model{
            cate_id:     params.cate_id,
            title:       params.title.clone(),
            keywords:    Some(params.keywords.clone()),
            description: Some(params.description.clone()),
            cover:       Some(params.cover.clone()),
            content:     params.content.clone(),
            html_content:params.html_content.clone(),
            brief:       Some(params.brief.clone()),
            tags:        Some(params.tags.clone()),
            from:        Some(params.from.clone()),
            is_top:      Some(params.is_top),
            status:      Some(params.status),
            add_time:    Some(add_time),
            ..entity::default()
        })
        .await;
    if data.is_err() {
        return Ok(nako_http::error_json("更新失败"));
    }

    Ok(nako_http::success_json("更新成功", ""))
}

// ==========================

#[derive(Deserialize)]
pub struct DeleteForm {
    id: u32,
}

// 删除
pub async fn delete(
    state: web::Data<AppState>,
    query: web::Form<DeleteForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if query.id == 0 {
        return Ok(nako_http::error_json("ID不能为空"));
    }

    let data = art::ArtModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_json("要删除的文章不存在"));
    }

    let delete_data = art::ArtModel::delete(db, query.id).await;
    if delete_data.is_err() {
        return Ok(nako_http::error_json("删除失败"));
    }

    Ok(nako_http::success_json("删除成功", ""))
}

pub async fn editor(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();
    let ctx = nako_http::view_data();
    Ok(nako_http::view(&mut view, "admin/art/editor.html", &ctx))
}


// ==========================

#[derive(Deserialize)]
pub struct UpdateStatusQuery {
    id: u32,
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateStatusForm {
    status: i32,
}

// 更新保存
pub async fn update_status(
    state: web::Data<AppState>,
    query: web::Query<UpdateStatusQuery>,
    params: web::Form<UpdateStatusForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if query.id == 0 {
        return Ok(nako_http::error_json("ID不能为空"));
    }

    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_json("状态不能为空"));
    }

    let data = art::ArtModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_json("要更改的文章不存在"));
    }

    // 更新
    let status = art::ArtModel::update_status_by_id(db, query.id, art_entity::Model{
            status: Some(params.status),
            ..entity::default()
        })
        .await;
    if status.is_err() {
        return Ok(nako_http::error_json("更新失败"));
    }

    Ok(nako_http::success_json("更新成功", ""))
}

