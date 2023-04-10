use actix_web::web;
mod heightmap_items;

pub fn heightmap_app_views_factory(app: &mut web::ServiceConfig) {
    app.route("/height_map_view", web::get().to(heightmap_items));
}
