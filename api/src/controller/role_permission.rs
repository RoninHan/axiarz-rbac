use api::tools::{AppState, FlashData, Params};
use rbac_service::{
    sea_orm::{Database, DatabaseConnection},
    Query as QueryCore, RolePermission as RolePermissionService,
};
use sea_orm::entity::prelude::*;

pub struct RolePermissionController;

impl RolePermissionController {
    pub async fn list_role_permissions(
        state: State<AppState>,
        Query(params): Query<Params>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (posts, num_pages) = QueryCore::find_posts_in_page(&state.conn, page, posts_per_page)
            .await
            .expect("Cannot find posts in page");

        let mut ctx = tera::Context::new();
        ctx.insert("posts", &posts);
        ctx.insert("page", &page);
        ctx.insert("posts_per_page", &posts_per_page);
        ctx.insert("num_pages", &num_pages);

        if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
            ctx.insert("flash", &value);
        }

        let body = state
            .templates
            .render("index.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn create_role_permission(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<role_permission::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        RolePermissionService::create_role_permission(&state.conn, form);

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Post created successfully".to_string(),
            },
        ));
    }

    pub async fn update_role_permission(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<role_permission::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        RolePermissionService::update_role_permission(&state.conn, form);

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Post updated successfully".to_string(),
            },
        ));
    }

    pub async fn delete_role_permission(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<role_permission::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        RolePermissionService::delete_role_permission(&state.conn, form);

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Post deleted successfully".to_string(),
            },
        ));
    }
}
