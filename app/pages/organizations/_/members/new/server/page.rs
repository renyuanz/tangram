use html::{component, html, Props};
use tangram_app_layouts::{
	app_layout::{AppLayout, AppLayoutProps},
	document::{Document, DocumentProps},
};
use tangram_ui as ui;

#[derive(Props)]
pub struct PageProps {
	pub app_layout_props: AppLayoutProps,
	pub error: Option<String>,
}

#[component]
pub fn Page(props: PageProps) {
	let document_props = DocumentProps {
		client_wasm_js_src: None,
	};
	html! {
		<Document {document_props}>
			<AppLayout {props.app_layout_props}>
				<ui::S1>
					<ui::H1>{"Invite"}</ui::H1>
					<ui::Form post?={Some(true)}>
						<ui::TextField
							label?="Email"
							name?="email"
						/>
						<ui::CheckboxField
							label?="Admin"
							name?="is_admin"
						/>
						<ui::Button button_type?={Some(ui::ButtonType::Submit)}>
							{"Invite"}
						</ui::Button>
					</ui::Form>
				</ui::S1>
			</AppLayout>
		</Document>
	}
}
