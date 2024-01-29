use yew::prelude::*;

const WIDTH_COLLAPSED: &'static str = "w-16";
const WIDTH_EXPANDED: &'static str = "w-64";
const PADDING_COLLAPSED: &'static str = "p-2";
const PADDING_EXPANDED: &'static str = "p-4";
const DISPLAY_COLLAPSED: &'static str = "hidden";
const DISPLAY_EXPANDED: &'static str = "flex";
const JUSTIFY_CONTENT: &'static str = "justify-start";
const ALIGN_ITEMS: &'static str = "items-center";
const HEIGHT: &'static str = "h-screen";
const BACKGROUND_COLOR: &'static str = "bg-gray-800";
const FONT: &'static str = "text-black";
const ICON_COLOR: &'static str = "white";
const BUTTON_BORDER_RADIUS: &'static str = "rounded";
const BUTTON_BACKGROUND_COLOR: &'static str = "bg-blue-600";
const BUTTON_WIDTH: &'static str = "w-12";
const BUTTON_HEIGHT: &'static str = "h-12";
const MENU_ITEM: &'static str = "\
    text-gray-300 \
    hover:bg-gray-800 \
    hover:text-white \
    flex \
    items-center \
    space-x-2 \
    p-1 \
    rounded \
    transition duration-300 \
";
const LOGO_CLASS: &str = "flex items-center";
const LOGO_IMG_CLASS: &str = "w-32 md:w-40";

#[derive(Properties, Clone, PartialEq)]
pub struct SidebarProps {
    // General Props
    #[prop_or(false)]
    pub fixed: bool,
    #[prop_or(false)]
    pub sider_collapsed: bool,
    #[prop_or_default]
    pub menu_items: Vec<MenuItem>,
    // Prop for toggle icon when collapsed
    #[prop_or_default]
    pub toggle_icon_collapsed: Html,
    // Prop for toggle icon when expanded
    #[prop_or_default]
    pub toggle_icon_expanded: Html,

    // Layout Props
    // Prop for width of collapsed state
    #[prop_or(WIDTH_COLLAPSED)]
    pub width_collapsed: &'static str,
    // Prop for width of expanded state
    #[prop_or(WIDTH_EXPANDED)]
    pub width_expanded: &'static str,
    // Prop for padding of collapsed state
    #[prop_or(PADDING_COLLAPSED)]
    pub padding_collapsed: &'static str,
    // Prop for padding of expanded state
    #[prop_or(PADDING_EXPANDED)]
    pub padding_expanded: &'static str,
    // Prop for display of collapsed state
    #[prop_or(DISPLAY_COLLAPSED)]
    pub display_collapsed: &'static str,
    // Prop for display of expanded state
    #[prop_or(DISPLAY_EXPANDED)]
    pub display_expanded: &'static str,
    // Prop for justify-content
    #[prop_or(JUSTIFY_CONTENT)]
    pub justify_content: &'static str,
    // Prop for align-items
    #[prop_or(ALIGN_ITEMS)]
    pub align_items: &'static str,
    // Prop for height
    #[prop_or(HEIGHT)]
    pub height: &'static str,

    // Style Props
    // Prop for background color
    #[prop_or(BACKGROUND_COLOR)]
    pub background_color: &'static str,
    // Prop for font
    #[prop_or(FONT)]
    pub font: &'static str,
    // Prop for icon color
    #[prop_or(ICON_COLOR)]
    pub icon_color: &'static str,
    // Prop for button border-radius
    #[prop_or(BUTTON_BORDER_RADIUS)]
    pub button_border_radius: &'static str,
    // Prop for button background color
    #[prop_or(BUTTON_BACKGROUND_COLOR)]
    pub button_background_color: &'static str,
    // Prop for button width
    #[prop_or(BUTTON_WIDTH)]
    pub button_width: &'static str,
    // Prop for button height
    #[prop_or(BUTTON_HEIGHT)]
    pub button_height: &'static str,

    // Title Props
    // Prop for title
    #[prop_or_default]
    pub title: &'static str,

    // Logo Props
    #[prop_or("images/logo.png")]
    pub logo_src: &'static str,
    #[prop_or("logo")]
    pub logo_alt: &'static str,
    #[prop_or(LOGO_IMG_CLASS)]
    pub logo_img_class: &'static str,
    #[prop_or("/")]
    pub logo_link: &'static str,
    #[prop_or(LOGO_CLASS)]
    pub logo_class: &'static str,

    // Bottom section props
    #[prop_or_default]
    pub bottom_section: Html,
}

impl Default for SidebarProps {
    fn default() -> Self {
        Self {
            fixed: false,
            sider_collapsed: false,
            title: "",
            menu_items: Vec::new(),
            width_collapsed: WIDTH_COLLAPSED,
            width_expanded: WIDTH_EXPANDED,
            padding_collapsed: PADDING_COLLAPSED,
            padding_expanded: PADDING_EXPANDED,
            display_collapsed: DISPLAY_COLLAPSED,
            display_expanded: DISPLAY_EXPANDED,
            justify_content: JUSTIFY_CONTENT,
            align_items: ALIGN_ITEMS,
            height: HEIGHT,
            background_color: BACKGROUND_COLOR,
            font: FONT,
            icon_color: ICON_COLOR,
            button_border_radius: BUTTON_BORDER_RADIUS,
            button_background_color: BUTTON_BACKGROUND_COLOR,
            button_width: BUTTON_WIDTH,
            button_height: BUTTON_HEIGHT,
            logo_src: "images/logo.png",
            logo_alt: "logo",
            logo_img_class: LOGO_CLASS,
            logo_link: "/",
            logo_class: LOGO_CLASS,
            toggle_icon_collapsed: html! {},
            toggle_icon_expanded: html! {},
            bottom_section: html! {},
        }
    }
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let is_collapsed_handle = use_state(|| props.sider_collapsed);
    let is_collapsed = (*is_collapsed_handle).clone();

    html! {
        <>
            { if props.fixed {
                html! {
                    <div
                        class={format!("transition-all duration-200 {}",
                            if is_collapsed { props.width_collapsed }
                            else { props.width_expanded })
                        }
                    />
                }
            } else {
                html! {}
            } }
            <div
                class={format!(
                    "{} {} {} {} {} {} {} {}",
                    if is_collapsed { props.width_collapsed } else { props.width_expanded },
                    if is_collapsed { props.padding_collapsed } else { props.padding_expanded },
                    props.display_expanded,
                    props.justify_content,
                    props.align_items,
                    props.height,
                    props.background_color,
                    props.font,
                )}
            >
                { render_logo_and_title(&props, is_collapsed_handle) }
                { render_menu(&props.menu_items, is_collapsed) }
                { props.bottom_section.clone() }
            </div>
        </>
    }
}

fn render_logo_and_title(props: &SidebarProps, is_collapsed_handle: UseStateHandle<bool>) -> Html {
    let on_toggle = {
        let is_collapsed_handle = is_collapsed_handle.clone();
        Callback::from(move |_| {
            is_collapsed_handle.set(!*is_collapsed_handle);
        })
    };
    let props_clone = props.clone();
    html! {
        <div class="flex items-center">
            <button
                type="button"
                class={format!(
                    "{} {} {} {}",
                    props.button_border_radius,
                    props.button_background_color,
                    props.button_width,
                    props.button_height,
                )}
                onclick={on_toggle}
            >
                { if *is_collapsed_handle {
                    props_clone.toggle_icon_collapsed
                } else {
                    props_clone.toggle_icon_expanded
                } }
            </button>
            if !*is_collapsed_handle && !props.logo_src.is_empty() { { render_logo(&props) } }
            if !*is_collapsed_handle { <span class="ml-2 text-white">{ props.title }</span> }
        </div>
    }
}

fn render_logo(props: &SidebarProps) -> Html {
    html! {
        <div id="logo" class={props.logo_class}>
            <a href={props.logo_link} class="nav-link">
                <img src={props.logo_src} alt={props.logo_alt} class={props.logo_img_class} />
            </a>
        </div>
    }
}

fn render_menu(menu_items: &[MenuItem], is_collapsed: bool) -> Html {
    html! { <ul>{ for menu_items.iter().map(|item| render_menu_item(item, is_collapsed)) }</ul> }
}

fn render_menu_item(item: &MenuItem, is_collapsed: bool) -> Html {
    html! {
        <li class="mb-1">
            <a href={item.link} class={MENU_ITEM}>
                { item.icon.clone() }
                if !is_collapsed { <span class="ml-2">{ &item.text }</span> }
                { if !item.submenus.is_empty() && !is_collapsed {
                    html! { <ul class={"ml-1"}>{render_submenu(&item.submenus)}</ul> }
                } else {
                    html! {}
                } }
            </a>
        </li>
    }
}

fn render_submenu(submenus: &[MenuItem]) -> Html {
    html! { { for submenus.iter().map(|submenu| render_menu_item(submenu, false)) } }
}

#[derive(Clone, Properties, PartialEq)]
pub struct MenuItem {
    pub icon: Html,
    pub text: &'static str,
    pub link: &'static str,
    #[prop_or_default]
    pub submenus: Vec<MenuItem>,
}
