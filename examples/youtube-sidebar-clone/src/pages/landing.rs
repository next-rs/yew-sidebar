use yew::prelude::*;
use yew_sidebar::{MenuItem, Sidebar, SidebarProps};

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let menu_items = vec![
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M4 21V10.08l8-6.96 8 6.96V21h-6v-6h-4v6H4z"></path>
                </svg>
            },
            text: "Home",
            link: "#home",
            title: "",
            class: "",
            submenus: vec![],
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M10 14.65v-5.3L15 12l-5 2.65zm7.77-4.33-1.2-.5L18 9.06c1.84-.96 2.53-3.23 1.56-5.06s-3.24-2.53-5.07-1.56L6 6.94c-1.29.68-2.07 2.04-2 3.49.07 1.42.93 2.67 2.22 3.25.03.01 1.2.5 1.2.5L6 14.93c-1.83.97-2.53 3.24-1.56 5.07.97 1.83 3.24 2.53 5.07 1.56l8.5-4.5c1.29-.68 2.06-2.04 1.99-3.49-.07-1.42-.94-2.68-2.23-3.25zm-.23 5.86-8.5 4.5c-1.34.71-3.01.2-3.72-1.14-.71-1.34-.2-3.01 1.14-3.72l2.04-1.08v-1.21l-.69-.28-1.11-.46c-.99-.41-1.65-1.35-1.7-2.41-.05-1.06.52-2.06 1.46-2.56l8.5-4.5c1.34-.71 3.01-.2 3.72 1.14.71 1.34.2 3.01-1.14 3.72L15.5 9.26v1.21l1.8.74c.99.41 1.65 1.35 1.7 2.41.05 1.06-.52 2.06-1.46 2.56z"></path>
                </svg>
            },
            text: "Shorts",
            link: "#shorts",
            submenus: vec![],
            class: "",
            title: "",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M10 18v-6l5 3-5 3zm7-15H7v1h10V3zm3 3H4v1h16V6zm2 3H2v12h20V9zM3 10h18v10H3V10z"></path>
                </svg>
            },
            text: "Subscriptions",
            link: "#subscriptions",
            submenus: vec![],
            class: "border-b pb-4 pr-2",
            title: "",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M3,3v18h18V3H3z M4.99,20c0.39-2.62,2.38-5.1,7.01-5.1s6.62,2.48,7.01,5.1H4.99z M9,10c0-1.65,1.35-3,3-3s3,1.35,3,3 c0,1.65-1.35,3-3,3S9,11.65,9,10z M12.72,13.93C14.58,13.59,16,11.96,16,10c0-2.21-1.79-4-4-4c-2.21,0-4,1.79-4,4 c0,1.96,1.42,3.59,3.28,3.93c-4.42,0.25-6.84,2.8-7.28,6V4h16v15.93C19.56,16.73,17.14,14.18,12.72,13.93z"></path>
                    </svg>
            },
            text: "Your channel",
            link: "#channel",
            submenus: vec![],
            class: "",
            title: "You>",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M14.97 16.95 10 13.87V7h2v5.76l4.03 2.49-1.06 1.7zM22 12c0 5.51-4.49 10-10 10S2 17.51 2 12h1c0 4.96 4.04 9 9 9s9-4.04 9-9-4.04-9-9-9C8.81 3 5.92 4.64 4.28 7.38c-.11.18-.22.37-.31.56L3.94 8H8v1H1.96V3h1v4.74c.04-.09.07-.17.11-.25.11-.22.23-.42.35-.63C5.22 3.86 8.51 2 12 2c5.51 0 10 4.49 10 10z"></path>
                </svg>
            },
            text: "History",
            link: "#history",
            submenus: vec![],
            class: "",
            title: "",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="m10 8 6 4-6 4V8zm11-5v18H3V3h18zm-1 1H4v16h16V4z"></path>
                </svg>
            },
            text: "Your Videos",
            link: "#videos",
            submenus: vec![],
            class: "",
            title: "",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M14.97 16.95 10 13.87V7h2v5.76l4.03 2.49-1.06 1.7zM12 3c-4.96 0-9 4.04-9 9s4.04 9 9 9 9-4.04 9-9-4.04-9-9-9m0-1c5.52 0 10 4.48 10 10s-4.48 10-10 10S2 17.52 2 12 6.48 2 12 2z"></path>
                </svg>
            },
            text: "Watch Later",
            link: "#watch-later",
            submenus: vec![],
            class: "",
            title: "",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M18.77,11h-4.23l1.52-4.94C16.38,5.03,15.54,4,14.38,4c-0.58,0-1.14,0.24-1.52,0.65L7,11H3v10h4h1h9.43 c1.06,0,1.98-0.67,2.19-1.61l1.34-6C21.23,12.15,20.18,11,18.77,11z M7,20H4v-8h3V20z M19.98,13.17l-1.34,6 C18.54,19.65,18.03,20,17.43,20H8v-8.61l5.6-6.06C13.79,5.12,14.08,5,14.38,5c0.26,0,0.5,0.11,0.63,0.3 c0.07,0.1,0.15,0.26,0.09,0.47l-1.52,4.94L13.18,12h1.35h4.23c0.41,0,0.8,0.17,1.03,0.46C19.92,12.61,20.05,12.86,19.98,13.17z"></path>
                </svg>
            },
            text: "Liked videos",
            link: "#liked-videos",
            submenus: vec![],
            class: "border-b pb-4",
            title: "",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M19 3.87v9.77C19 17.7 15.86 21 12 21s-7-3.3-7-7.37v-.13c0-1.06.22-2.13.62-3.09.5-1.19 1.29-2.21 2.27-2.97.85-.66 1.83-1.14 2.87-1.65.39-.19.77-.38 1.15-.58.36-.19.72-.38 1.08-.56v3.22l1.55-1.04L19 3.87M20 2l-6 4V3c-.85.44-1.7.88-2.55 1.33-1.41.74-2.9 1.34-4.17 2.32-1.13.87-2.02 2.05-2.58 3.37-.46 1.09-.7 2.29-.7 3.48v.14C4 18.26 7.58 22 12 22s8-3.74 8-8.36V2zM9.45 12.89 14 10v5.7c0 1.82-1.34 3.3-3 3.3s-3-1.47-3-3.3c0-1.19.58-2.23 1.45-2.81z"></path>
                </svg>
            },
            text: "Trending",
            link: "#trending",
            submenus: vec![],
            class: "",
            title: "Explore",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M12 4v9.38c-.73-.84-1.8-1.38-3-1.38-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V8h6V4h-7zM9 19c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm9-12h-5V5h5v2z"></path>
                </svg>
            },
            text: "Music",
            link: "#music",
            submenus: vec![],
            class: "",
            title: "",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M10 12H8v2H6v-2H4v-2h2V8h2v2h2v2zm7 .5c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5.67 1.5 1.5 1.5 1.5-.67 1.5-1.5zm3-3c0-.83-.67-1.5-1.5-1.5S17 8.67 17 9.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5zm-3.03-4.35-4.5 2.53-.49.27-.49-.27-4.5-2.53L3 7.39v6.43l8.98 5.04 8.98-5.04V7.39l-3.99-2.24m0-1.15 4.99 2.8v7.6L11.98 20 2 14.4V6.8L6.99 4l4.99 2.8L16.97 4z"></path>
                </svg>
            },
            text: "Gaming",
            link: "#gaming",
            submenus: vec![],
            class: "",
            title: "",
        },
        MenuItem {
            icon: html! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="w-6 h-6 text-white">
                    <path d="M18 5V2H6v3H3v6l3.23 1.61c.7 2.5 2.97 4.34 5.69 4.38L8 19v3h8v-3l-3.92-2.01c2.72-.04 4.99-1.88 5.69-4.38L21 11V5h-3zM6 11.38l-2-1V6h2v5.38zM15 21H9v-1.39l3-1.54 3 1.54V21zm2-10c0 2.76-2.24 5-5 5s-5-2.24-5-5V3h10v8zm3-.62-2 1V6h2v4.38z"></path>
                </svg>
            },
            text: "Sports",
            link: "#sports",
            submenus: vec![],
            class: "border-b pb-4",
            title: "",
        },
    ];
    let toggle_icon_collapsed = html! {
        <svg
            viewBox="0 0 24 24"
            aria-hidden="true"
            fill="none"
            viewBox="0 0 24 24"
            stroke="white"
            class="p-2 pl-1 mr-3"
        ><path d="M21 6H3V5h18v1zm0 5H3v1h18v-1zm0 6H3v1h18v-1z" /></svg>
    };
    let toggle_icon_expanded = html! {
        <svg
            viewBox="0 0 24 24"
            aria-hidden="true"
            fill="none"
            viewBox="0 0 24 24"
            stroke="white"
            class="p-2 pl-1 mr-3"
        ><path d="M21 6H3V5h18v1zm0 5H3v1h18v-1zm0 6H3v1h18v-1z" /></svg>
    };
    let sidebar_props = SidebarProps {
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        logo_src: "images/logo.png",
        logo_img_class: "w-20",
        sider_collapsed: false,
        width_collapsed: "w-26",
        title: "",
        width_expanded: "w-48",
        padding_collapsed: "p-5",
        justify_content: "flex-col",
        padding_expanded: "p-4",
        font: "text-lg text-white",
        display_collapsed: "hidden",
        button_background_color: "transparent pl-1",
        display_expanded: "flex",
        align_items: "items-start",
        height: "h-screen",
        button_height: "h-16",
        menu_items: menu_items.clone(),
        background_color: "bg-black",
        ..SidebarProps::default()
    };

    let sidebar_props_1 = SidebarProps {
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        logo_src: "images/logo.png",
        logo_img_class: "w-20 animate-pulse",
        sider_collapsed: false,
        width_collapsed: "w-24",
        title: "",
        width_expanded: "w-64",
        padding_collapsed: "p-5",
        justify_content: "flex-col",
        padding_expanded: "p-4",
        font: "text-lg text-white",
        display_collapsed: "hidden",
        button_background_color: "",
        display_expanded: "flex",
        align_items: "items-start",
        height: "h-screen",
        button_height: "h-16",
        menu_items: menu_items.clone(),
        background_color: "bg-blue-800",
        ..SidebarProps::default()
    };

    let sidebar_props_2 = SidebarProps {
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        logo_src: "images/logo.png",
        logo_img_class: "w-20 animate-spin",
        sider_collapsed: false,
        width_collapsed: "w-20",
        title: "",
        width_expanded: "w-48",
        padding_collapsed: "p-5",
        justify_content: "flex-col",
        padding_expanded: "p-4",
        font: "text-lg text-white",
        display_collapsed: "hidden",
        button_background_color: "",
        display_expanded: "flex",
        align_items: "items-start",
        height: "h-screen",
        button_height: "h-16",
        menu_items: menu_items.clone(),
        background_color: "bg-gradient-to-r from-blue-500 to-blue-800",
        ..SidebarProps::default()
    };

    let sidebar_props_3 = SidebarProps {
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        logo_src: "images/logo.png",
        logo_img_class: "w-20 animate-bounce",
        sider_collapsed: false,
        width_collapsed: "w-20",
        title: "",
        width_expanded: "w-64",
        padding_collapsed: "p-5",
        justify_content: "flex-col",
        padding_expanded: "p-4",
        font: "text-lg text-white",
        display_collapsed: "hidden",
        button_background_color: "",
        display_expanded: "flex",
        align_items: "items-start",
        height: "h-screen",
        button_height: "h-16",
        menu_items: menu_items.clone(),
        background_color: "rounded border-2 bg-gray-900",
        ..SidebarProps::default()
    };
    let sidebar_props_4 = SidebarProps {
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        logo_src: "images/logo.png",
        logo_img_class: "w-20 animate-ping",
        sider_collapsed: false,
        width_collapsed: "w-20",
        title: "",
        width_expanded: "w-48",
        padding_collapsed: "p-5",
        justify_content: "flex-col",
        padding_expanded: "p-4",
        font: "text-lg text-white",
        display_collapsed: "hidden",
        button_background_color: "",
        display_expanded: "flex",
        align_items: "items-start",
        height: "h-screen",
        button_height: "h-16",
        menu_items: menu_items.clone(),
        background_color: "bg-blue-800 animate-pulse",
        ..SidebarProps::default()
    };
    let sidebar_props_5 = SidebarProps {
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        logo_src: "images/logo.png",
        logo_img_class: "w-20 animate-heartbeat",
        sider_collapsed: false,
        width_collapsed: "w-20",
        title: "",
        width_expanded: "w-48",
        padding_collapsed: "p-5",
        justify_content: "flex-col",
        padding_expanded: "p-4",
        font: "text-lg text-white",
        display_collapsed: "hidden",
        button_background_color: "",
        display_expanded: "flex",
        align_items: "items-start",
        height: "h-screen",
        button_height: "h-16",
        menu_items: menu_items.clone(),
        background_color: "bg-orange-800",
        ..SidebarProps::default()
    };
    let sidebar_props_6 = SidebarProps {
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        logo_src: "images/logo.png",
        logo_img_class: "w-20",
        sider_collapsed: false,
        width_collapsed: "w-20",
        title: "",
        width_expanded: "w-48",
        padding_collapsed: "p-5",
        justify_content: "flex-col",
        padding_expanded: "p-4",
        font: "text-lg text-white",
        display_collapsed: "hidden",
        button_background_color: "hover:bg-purple-700",
        display_expanded: "flex",
        align_items: "items-start",
        height: "h-screen",
        button_height: "h-16",
        menu_items: menu_items.clone(),
        background_color: "bg-gradient-to-r from-purple-500 to-purple-800",
        ..SidebarProps::default()
    };
    html! {
        <div class="flex">
            <Sidebar ..sidebar_props />
            <Sidebar ..sidebar_props_1 />
            <Sidebar ..sidebar_props_2 />
            <Sidebar ..sidebar_props_3 />
            <Sidebar ..sidebar_props_4 />
            <Sidebar ..sidebar_props_5 />
            <Sidebar ..sidebar_props_6 />
        </div>
    }
}
