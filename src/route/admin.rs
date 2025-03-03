use actix_web::web;
use actix_web_lab::middleware::from_fn;

use crate::nako::{
    config,
};

use crate::app::middleware::{
    admin_auth,
};
use crate::app::controller::admin::{
    index,
    auth,
    user,
    cate,
    profile,
    upload,
    attach,
    comment,
    tag,
    art,
    page,
    setting,
    error,
    guestbook,
    friendlink,
};

pub fn route(cfg: &mut web::ServiceConfig) {
    let admin_prefix = config::section::<String>("app", "admin_prefix", "admin".to_string());

    cfg.service(
        web::scope(admin_prefix.as_str())
            .service(
                // 后台首页
                web::scope("/index")
                    .service(
                        web::resource("")
                            .route(web::get().to(index::index))
                            .name("admin.index"),
                    )
                    .service(
                        web::resource("/console")
                            .route(web::get().to(index::console))
                            .name("admin.index-console"),
                    )
            )
            .service(
                // 登陆相关
                web::scope("/auth")
                    .service(
                        web::resource("/captcha")
                            .route(web::get().to(auth::captcha))
                            .name("admin.auth-captcha"),
                    )
                    .service(
                        web::resource("/login")
                            .route(web::get().to(auth::login))
                            .route(web::post().to(auth::login_check))
                            .name("admin.auth-login"),
                    )
                    .service(
                        web::resource("/logout")
                            .route(web::get().to(auth::logout))
                            .name("admin.auth-logout"),
                    )
            )
            .service(
                // 个人信息
                web::scope("/profile")
                    .service(
                        web::resource("/info")
                            .route(web::get().to(profile::update_info))
                            .route(web::post().to(profile::update_info_save))
                            .name("admin.profile-info"),
                    )
                    .service(
                        web::resource("/password")
                            .route(web::get().to(profile::update_password))
                            .route(web::post().to(profile::update_password_save))
                            .name("admin.profile-password"),
                    )
                    .service(
                        web::resource("/avatar")
                            .route(web::get().to(profile::update_avatar))
                            .route(web::post().to(profile::update_avatar_save))
                            .name("admin.profile-avatar"),
                    )
            )
            .service(
                // 上传
                web::scope("/upload")
                    .service(
                        web::resource("/image")
                            .route(web::post().to(upload::image))
                            .name("admin.upload-image"),
                    )
                    .service(
                        web::resource("/file")
                            .route(web::post().to(upload::file))
                            .name("admin.upload-file"),
                    )
                    .service(
                        web::resource("/avatar")
                            .route(web::post().to(upload::avatar))
                            .name("admin.upload-avatar"),
                    )
            )
            .service(
                // 文章
                web::scope("/art")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(art::index))
                            .name("admin.art-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(art::list))
                            .name("admin.art-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(art::detail))
                            .name("admin.art-detail"),
                    )
                    .service(
                        web::resource("/create")
                            .route(web::get().to(art::create))
                            .route(web::post().to(art::create_save))
                            .name("admin.art-create"),
                    )
                    .service(
                        web::resource("/update")
                            .route(web::get().to(art::update))
                            .route(web::post().to(art::update_save))
                            .name("admin.art-update"),
                    )
                    .service(
                        web::resource("/status")
                            .route(web::post().to(art::update_status))
                            .name("admin.art-status"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(art::delete))
                            .name("admin.art-delete"),
                    )
                    .service(
                        web::resource("/editor")
                            .route(web::get().to(art::editor))
                            .name("admin.art-editor"),
                    )
            )
            .service(
                // 分类
                web::scope("/cate")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(cate::index))
                            .name("admin.cate-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(cate::list))
                            .name("admin.cate-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(cate::detail))
                            .name("admin.cate-detail"),
                    )
                    .service(
                        web::resource("/create")
                            .route(web::get().to(cate::create))
                            .route(web::post().to(cate::create_save))
                            .name("admin.cate-create"),
                    )
                    .service(
                        web::resource("/update")
                            .route(web::get().to(cate::update))
                            .route(web::post().to(cate::update_save))
                            .name("admin.cate-update"),
                    )
                    .service(
                        web::resource("/status")
                            .route(web::post().to(cate::update_status))
                            .name("admin.cate-status"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(cate::delete))
                            .name("admin.cate-delete"),
                    )
            )
            .service(
                // 评论
                web::scope("/comment")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(comment::index))
                            .name("admin.comment-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(comment::list))
                            .name("admin.comment-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(comment::detail))
                            .name("admin.comment-detail"),
                    )
                    .service(
                        web::resource("/status")
                            .route(web::post().to(comment::update_status))
                            .name("admin.comment-status"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(comment::delete))
                            .name("admin.comment-delete"),
                    )
                    .service(
                        web::resource("/batch-delete")
                            .route(web::post().to(comment::batch_delete))
                            .name("admin.comment-batch-delete"),
                    )
            )
            .service(
                // 标签
                web::scope("/tag")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(tag::index))
                            .name("admin.tag-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(tag::list))
                            .name("admin.tag-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(tag::detail))
                            .name("admin.tag-detail"),
                    )
                    .service(
                        web::resource("/create")
                            .route(web::get().to(tag::create))
                            .route(web::post().to(tag::create_save))
                            .name("admin.tag-create"),
                    )
                    .service(
                        web::resource("/update")
                            .route(web::get().to(tag::update))
                            .route(web::post().to(tag::update_save))
                            .name("admin.tag-update"),
                    )
                    .service(
                        web::resource("/status")
                            .route(web::post().to(tag::update_status))
                            .name("admin.tag-status"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(tag::delete))
                            .name("admin.tag-delete"),
                    )
            )
            .service(
                // 页面
                web::scope("/page")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(page::index))
                            .name("admin.page-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(page::list))
                            .name("admin.page-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(page::detail))
                            .name("admin.page-detail"),
                    )
                    .service(
                        web::resource("/create")
                            .route(web::get().to(page::create))
                            .route(web::post().to(page::create_save))
                            .name("admin.page-create"),
                    )
                    .service(
                        web::resource("/update")
                            .route(web::get().to(page::update))
                            .route(web::post().to(page::update_save))
                            .name("admin.page-update"),
                    )
                    .service(
                        web::resource("/status")
                            .route(web::post().to(page::update_status))
                            .name("admin.page-status"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(page::delete))
                            .name("admin.page-delete"),
                    )
            )
            .service(
                // 留言
                web::scope("/guestbook")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(guestbook::index))
                            .name("admin.guestbook-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(guestbook::list))
                            .name("admin.guestbook-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(guestbook::detail))
                            .name("admin.guestbook-detail"),
                    )
                    .service(
                        web::resource("/status")
                            .route(web::post().to(guestbook::update_status))
                            .name("admin.guestbook-status"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(guestbook::delete))
                            .name("admin.guestbook-delete"),
                    )
                    .service(
                        web::resource("/batch-delete")
                            .route(web::post().to(guestbook::batch_delete))
                            .name("admin.guestbook-batch-delete"),
                    )
            )
            .service(
                // 友情链接
                web::scope("/friendlink")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(friendlink::index))
                            .name("admin.friendlink-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(friendlink::list))
                            .name("admin.friendlink-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(friendlink::detail))
                            .name("admin.friendlink-detail"),
                    )
                    .service(
                        web::resource("/create")
                            .route(web::get().to(friendlink::create))
                            .route(web::post().to(friendlink::create_save))
                            .name("admin.friendlink-create"),
                    )
                    .service(
                        web::resource("/update")
                            .route(web::get().to(friendlink::update))
                            .route(web::post().to(friendlink::update_save))
                            .name("admin.friendlink-update"),
                    )
                    .service(
                        web::resource("/status")
                            .route(web::post().to(friendlink::update_status))
                            .name("admin.friendlink-status"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(friendlink::delete))
                            .name("admin.friendlink-delete"),
                    )
            )
            .service(
                // 用户
                web::scope("/user")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(user::index))
                            .name("admin.user-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(user::list))
                            .name("admin.user-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(user::detail))
                            .name("admin.user-detail"),
                    )
                    .service(
                        web::resource("/create")
                            .route(web::get().to(user::create))
                            .route(web::post().to(user::create_save))
                            .name("admin.user-create"),
                    )
                    .service(
                        web::resource("/update")
                            .route(web::get().to(user::update))
                            .route(web::post().to(user::update_save))
                            .name("admin.user-update"),
                    )
                    .service(
                        web::resource("/status")
                            .route(web::post().to(user::update_status))
                            .name("admin.user-status"),
                    )
                    .service(
                        web::resource("/update-password")
                            .route(web::get().to(user::update_password))
                            .route(web::post().to(user::update_password_save))
                            .name("admin.user-update-password"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(user::delete))
                            .name("admin.user-delete"),
                    )
            )
            .service(
                // 附件
                web::scope("/attach")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(attach::index))
                            .name("admin.attach-index"),
                    )
                    .service(
                        web::resource("/list")
                            .route(web::get().to(attach::list))
                            .name("admin.attach-list"),
                    )
                    .service(
                        web::resource("/detail")
                            .route(web::get().to(attach::detail))
                            .name("admin.attach-detail"),
                    )
                    .service(
                        web::resource("/delete")
                            .route(web::post().to(attach::delete))
                            .name("admin.attach-delete"),
                    )
                    .service(
                        web::resource("/download")
                            .route(web::get().to(attach::download))
                            .name("admin.attach-download"),
                    )
                    .service(
                        web::resource("/preview")
                            .route(web::get().to(attach::preview))
                            .name("admin.attach-preview"),
                    )
            )
            .service(
                // 设置
                web::scope("/setting")
                    .service(
                        web::resource("/index")
                            .route(web::get().to(setting::index))
                            .route(web::post().to(setting::setting_save))
                            .name("admin.setting-index"),
                    )
            )
            .default_service(web::to(error::index))
            .wrap(from_fn(admin_auth::auth)),
    );
}
