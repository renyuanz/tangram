use pinwheel::prelude::*;
use tangram_app_ui::logo::{Logo, LogoColorScheme};
use tangram_ui as ui;

#[derive(children, Default, new)]
#[new(default)]
pub struct AuthLayout {
	pub children: Vec<Node>,
}

impl Component for AuthLayout {
	fn into_node(self) -> Node {
		div()
			.class("auth-layout")
			.child(
				div()
					.class("auth-layout-logo-wrapper")
					.child(Logo::new().color_scheme(LogoColorScheme::Multi)),
			)
			.child(
				div()
					.class("auth-layout-card-wrapper")
					.child(ui::Card::new().child(self.children)),
			)
			.into_node()
	}
}
