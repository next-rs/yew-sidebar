# 📁 Yew Sidebar

[![Crates.io](https://img.shields.io/crates/v/yew-sidebar)](https://crates.io/crates/yew-sidebar)
[![Crates.io Downloads](https://img.shields.io/crates/d/yew-sidebar)](https://crates.io/crates/yew-sidebar)
![Crates.io License](https://img.shields.io/crates/l/yew-sidebar)
![Rust](https://img.shields.io/badge/rust-stable-orange)

---

![Demo](https://github.com/wiseaidev/yew-sidebar/assets/62179149/3e322002-844f-4abd-b4da-ba985ab127cb)

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
       let sidebar_props = SidebarProps {
           sider_collapsed: false,
           width_collapsed: "w-16",
           title: "Multi Level Menu",
           width_expanded: "w-64",
           padding_collapsed: "p-2",
           justify_content: "flex-col",
           padding_expanded: "p-4",
           font_size: "text-lg",
           menu_items: vec![
               MenuItem {
                   icon: html! {
                       <svg
                           xmlns="http://www.w3.org/2000/svg"
                           fill="none"
                           viewBox="0 0 24 24"
                           stroke="currentColor"
                           class="w-6 h-6"
                       >
                           <path
                               stroke-linecap="round"
                               stroke-linejoin="round"
                               stroke-width="2"
                               d="M6 4v16m6-16v16m6-16v16"
                           />
                       </svg>
                   },
                   text: "Home",
                   link: "#home",
                   submenus: vec![],
               },
           ],
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

### Styling Props

| Name                   | Type            | Default Value       | Description                                       |
| ---------------------- | --------------- | ------------------- | ------------------------------------------------- |
| `font_size`            | `&'static str`  | `"text-white"`      | Font size of the sidebar text.                    |
| `icon_color`           | `&'static str`  | `"text-white"`      | Color of the icons in the sidebar.                |
| `button_border_radius` | `&'static str`  | `"rounded"`         | Border radius of the sidebar button.             |
| `button_background_color` | `&'static str` | `"bg-blue-600"`     | Background color of the sidebar button.          |
| `button_width`         | `&'static str`  | `"w-12"`            | Width of the sidebar button.                      |
| `button_height`        | `&'static str`  | `"h-12"`            | Height of the sidebar button.                     |

## 📙 Examples

If you're curious about how to use it with different styling or additional features, you can check out the [examples folder](examples/tailwind) for more information.

## 🤝 Contribution

We welcome contributions from the community to enhance this Sidebar component. Feel free to open issues, submit pull requests, or provide feedback. Let's collaborate to make navigation in Yew even more stylish and functional!

## 📜 License

Yew Sidebar is licensed under the `MIT` License, allowing you to use, modify, and distribute it freely. Refer to the [`LICENSE`](LICENSE) file for more details.
