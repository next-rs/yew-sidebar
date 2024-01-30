# 📁 Yew Sidebar

[![Crates.io](https://img.shields.io/crates/v/yew-sidebar)](https://crates.io/crates/yew-sidebar)
[![Crates.io Downloads](https://img.shields.io/crates/d/yew-sidebar)](https://crates.io/crates/yew-sidebar)
![Crates.io License](https://img.shields.io/crates/l/yew-sidebar)
![Rust](https://img.shields.io/badge/rust-stable-orange)

---

| Example | Demo | URL |
| --- | --- | --- |
| Basic | ![Demo](https://github.com/next-rs/yew-sidebar/assets/62179149/c25ea59c-530e-416e-94fa-e154fb461fd4) | [![Netlify Status](https://api.netlify.com/api/v1/badges/0f5cbba1-4179-45c7-91e2-5540f3539a12/deploy-status)](https://yew-sidebar.netlify.app/) |
| X Clone | ![Demo](https://github.com/next-rs/yew-sidebar/assets/62179149/da35077c-0969-46b0-8fa6-b296909875b0) | [![Netlify Status](https://api.netlify.com/api/v1/badges/0f5cbba1-4179-45c7-91e2-5540f3539a12/deploy-status)](https://yew-sidebar-x.netlify.app/) |
| Youtube Clone | ![Demo](https://github.com/next-rs/yew-sidebar/assets/62179149/6133ada6-5870-42a8-9afd-a41c3f20f07d) | [![Netlify Status](https://api.netlify.com/api/v1/badges/0f5cbba1-4179-45c7-91e2-5540f3539a12/deploy-status)](https://yew-sidebar-youtube.netlify.app/) |

---

## 📜 Introduction

Yew Sidebar is a powerful and responsive sidebar component for the Yew framework. It provides a feature-rich sidebar that can be easily integrated into your Yew applications.

## 🤔 Why is this Component Useful?

This sidebar component offers several benefits to enhance your web application's navigation experience:

1. 🌐 Customization: Tailor the appearance and behavior of the sidebar to suit your application's design.

1. 🚀 Responsive Design: Ensures optimal viewing and navigation across various devices and screen sizes.

1. 💬 Easy Integration: Seamless integration into Yew projects with minimal setup and configuration.

## ⚙️ Installation

Integrating Yew Sidebar into your Yew project is a straightforward process. Follow these steps:

1. Make sure you have Yew set up in your project. If not, refer to the [Yew documentation](https://yew.rs/docs/getting-started/introduction) for installation instructions.

1. Install the library using your preferred package manager:

   ```bash
   $ cargo add yew-sidebar
   ```

1. Start using the sidebar component to enhance your application's navigation.

## 🛠️ Usage

Incorporating Yew Sidebar into your application is easy. Follow these steps:

1. Import the sidebar component into your Yew project:

   ```rust
   use yew::prelude::*;
   use yew_sidebar::{Sidebar, SidebarProps, MenuItem};

   #[function_component(App)]
   pub fn app() -> Html {
       // Tailwind css utility classes
        let toggle_icon_collapsed = html! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke="white"
                class="m-3 w-6 h-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M6 4v16m6-16v16m6-16v16"
                />
            </svg>
        };
        let toggle_icon_expanded = html! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke="white"
                class="m-3 w-6 h-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 6h16M4 12h16m-7 6h7"
                />
            </svg>
        };
        let sidebar_props = SidebarProps {
            sider_collapsed: true,
            title: "Dashboard",
            menu_items: vec![
                MenuItem {
                    icon: html! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                        </svg>
                    },
                    text: "Overview",
                    link: "#overview",
                    class: "",
                    title: "",
                    submenus: vec![],
                },
                MenuItem {
                    icon: html! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 4v16m6-16v16m6-16v16"></path>
                        </svg>
                    },
                    text: "Analytics",
                    link: "#analytics",
                    class: "",
                    title: "",
                    submenus: vec![],
                },
                MenuItem {
                    icon: html! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10V3L6 14h7v7l9-11h-7z"></path>
                        </svg>
                    },
                    text: "Reports",
                    link: "#reports",
                    class: "",
                    title: "",
                    submenus: vec![],
                },
            ],
            width_collapsed: "w-20",
            toggle_icon_expanded: toggle_icon_expanded,
            toggle_icon_collapsed: toggle_icon_collapsed,
            width_expanded: "w-72",
            padding_collapsed: "p-2",
            padding_expanded: "p-4",
            display_collapsed: "hidden",
            display_expanded: "flex",
            justify_content: "flex-col",
            align_items: "items-start",
            height: "h-screen",
            background_color: "bg-gray-800",
            font: "text-white",
            icon_color: "yellow",
            button_border_radius: "rounded-md",
            button_background_color: "bg-blue-600",
            button_width: "w-3/4",
            button_height: "h-10",
            ..SidebarProps::default()
        };

       html! {
           <Sidebar ..sidebar_props />
       }
   }
   ```

1. Customize the sidebar appearance and behavior using provided props.

1. Enjoy an enhanced navigation experience with Yew Sidebar.

## 🔧 Props

### Main Props

| Name                   | Type            | Default Value       | Description                                       |
| ---------------------- | --------------- | ------------------- | ------------------------------------------------- |
| `fixed`                | `bool`          | `false`             | Set whether the sidebar is fixed.                  |
| `sider_collapsed`      | `bool`          | `false`             | Initial collapsed state of the sidebar.            |
| `menu_items`           | `Vec<MenuItem>` | `Vec::new()`        | List of navigation menu items.                     |
| `toggle_icon_collapsed` | `Html`         | `Html::default()`   | Toggle icon when the sidebar is collapsed.         |
| `toggle_icon_expanded`  | `Html`         | `Html::default()`   | Toggle icon when the sidebar is expanded.          |

### Layout Props

| Name                   | Type            | Default Value       | Description                                       |
| ---------------------- | --------------- | ------------------- | ------------------------------------------------- |
| `width_collapsed`      | `&'static str`  | `"w-16"`            | Width of the sidebar in the collapsed state.       |
| `width_expanded`       | `&'static str`  | `"w-64"`            | Width of the sidebar in the expanded state.        |
| `padding_collapsed`    | `&'static str`  | `"p-2"`             | Padding of the sidebar in the collapsed state.    |
| `padding_expanded`     | `&'static str`  | `"p-4"`             | Padding of the sidebar in the expanded state.     |
| `display_collapsed`    | `&'static str`  | `"hidden"`          | Display property of the sidebar in the collapsed state. |
| `display_expanded`     | `&'static str`  | `"flex"`            | Display property of the sidebar in the expanded state.  |
| `justify_content`      | `&'static str`  | `"justify-start"`   | Justification of the sidebar content.             |
| `align_items`          | `&'static str`  | `"items-center"`    | Alignment of items in the sidebar.                 |
| `height`               | `&'static str`  | `"h-screen"`        | Height of the sidebar.                            |
| `background_color`     | `&'static str`  | `"bg-gray-800"`     | Background color of the sidebar.                   |

### Style Props

| Name                   | Type            | Default Value       | Description                                       |
| ---------------------- | --------------- | ------------------- | ------------------------------------------------- |
| `font`                 | `&'static str`  | `"text-white"`      | Font color of the sidebar text.                   |
| `icon_color`           | `&'static str`  | `"text-white"`      | Color of the icons in the sidebar.                |
| `button_border_radius` | `&'static str`  | `"rounded"`         | Border radius of the sidebar button.             |
| `button_background_color` | `&'static str` | `"bg-blue-600"`     | Background color of the sidebar button.          |
| `button_width`         | `&'static str`  | `"w-12"`            | Width of the sidebar button.                      |
| `button_height`        | `&'static str`  | `"h-12"`            | Height of the sidebar button.                     |

### Title Props

| Name                   | Type            | Default Value       | Description                                       |
| ---------------------- | --------------- | ------------------- | ------------------------------------------------- |
| `title`                | `&'static str`  | `""`                | Title of the sidebar.                             |

### Logo Props

| Name                   | Type            | Default Value       | Description                                       |
| ---------------------- | --------------- | ------------------- | ------------------------------------------------- |
| `logo_src`             | `&'static str`  | `"images/logo.png"` | Source URL for the logo image.                    |
| `logo_alt`             | `&'static str`  | `"logo"`            | Alternative text for the logo.                    |
| `logo_img_class`       | `&'static str`  | `LOGO_IMG_CLASS`    | Class for styling the logo image.                 |
| `logo_link`            | `&'static str`  | `"/"`               | Link for the logo.                                |
| `logo_class`           | `&'static str`  | `LOGO_CLASS`        | Class for styling the logo.                       |

### Bottom Section Props

| Name                   | Type            | Default Value       | Description                                       |
| ---------------------- | --------------- | ------------------- | ------------------------------------------------- |
| `bottom_section`       | `Html`          | `Html::default()`   | Content for the bottom section of the sidebar.    |

### Submenus Props

| Name                   | Type            | Default Value       | Description                                       |
| ---------------------- | --------------- | ------------------- | ------------------------------------------------- |
| `size`                 | `&'static str`  | `""`                | Size of the accordion. Possible values: "sm", "md", "lg". |
| `aria_controls`        | `&'static str`  | `""`                | ARIA controls attribute for accessibility.       |
| `container_class`      | `&'static str`  | `""`                | Class for the container element.                  |
| `expanded_element_class` | `&'static str` | `""`                | Class for the expanded element.                   |
| `collapsed_element_class` | `&'static str` | `""`                | Class for the collapsed element.                  |
| `content_container_class` | `&'static str` | `""`                | Class for the content container.                  |

## 📙 Examples

If you're curious about how to use it with different styling or additional features, you can check out the [examples folder](examples) for more information.

## 🤝 Contribution

We welcome contributions from the community to enhance this Sidebar component. Feel free to open issues, submit pull requests, or provide feedback. Let's collaborate to make navigation in Yew even more stylish and functional!

## 📜 License

Yew Sidebar is licensed under the `MIT` License, allowing you to use, modify, and distribute it freely. Refer to the [`LICENSE`](LICENSE) file for more details.
