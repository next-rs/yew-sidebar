use yew::prelude::*;
use yew_sidebar::{MenuItem, Sidebar, SidebarProps};

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
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
        logo_src: "images/logo.png",
        sider_collapsed: false,
        width_collapsed: "w-16",
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        title: "Multi Level Menu",
        width_expanded: "w-64",
        padding_collapsed: "p-2",
        justify_content: "flex-col",
        padding_expanded: "p-4",
        font: "text-lg",
        display_collapsed: "hidden",
        display_expanded: "flex",
        align_items: "items-start",
        height: "h-screen",
        button_height: "h-12",
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
                class: "",
                title: "",
                submenus: vec![],
            },
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
                            d="M4 6h16M4 12h16m-7 6h7"
                        />
                    </svg>
                },
                text: "About",
                link: "#about",
                class: "",
                title: "",
                submenus: vec![
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
                                    d="M13 10V3L4 14h7v7l9-11h-7z"
                                />
                            </svg>
                        },
                        text: "Team",
                        link: "#team",
                        class: "",
                        title: "",
                        submenus: vec![],
                    },
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
                                    d="M15 10V3L6 14h7v7l9-11h-7z"
                                />
                            </svg>
                        },
                        text: "History",
                        link: "#history",
                        class: "",
                        title: "",
                        submenus: vec![],
                    },
                ],
            },
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
                            d="M15 10V3L6 14h7v7l9-11h-7z"
                        />
                    </svg>
                },
                text: "Services",
                link: "#services",
                class: "",
                title: "",
                submenus: vec![
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
                        text: "Web Design",
                        link: "#web-design",
                        class: "",
                        title: "",
                        submenus: vec![],
                    },
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
                                    d="M4 6h16M4 12h16m-7 6h7"
                                />
                            </svg>
                        },
                        text: "Graphic Design",
                        link: "#graphic-design",
                        class: "",
                        title: "",
                        submenus: vec![],
                    },
                ],
            },
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
                            d="M4 6h16M4 12h16m-7 6h7"
                        />
                    </svg>
                },
                text: "Contact",
                link: "#contact",
                class: "",
                title: "",
                submenus: vec![],
            },
        ],
        ..SidebarProps::default()
    };

    let sidebar_props1 = SidebarProps {
        fixed: false,
        logo_src: "",
        sider_collapsed: false,
        title: "Main Menu",
        menu_items: vec![
            MenuItem {
                icon: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                    </svg>
                },
                text: "Home",
                link: "#home",
                class: "",
                title: "",
                submenus: vec![],
            },
            MenuItem {
                icon: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"></path>
                    </svg>
                },
                text: "About",
                link: "#about",
                class: "",
                title: "",
                submenus: vec![
                    MenuItem {
                        icon: html! {
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                            </svg>
                        },
                        text: "Team",
                        link: "#team",
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
                        text: "History",
                        link: "#history",
                        class: "",
                        title: "",
                        submenus: vec![],
                    },
                ],
            },
            MenuItem {
                icon: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10V3L6 14h7v7l9-11h-7z"></path>
                    </svg>
                },
                text: "Services",
                link: "#services",
                class: "",
                title: "",
                submenus: vec![
                    MenuItem {
                        icon: html! {
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 4v16m6-16v16m6-16v16"></path>
                            </svg>
                        },
                        text: "Web Design",
                        link: "#web-design",
                        class: "",
                        title: "",
                        submenus: vec![],
                    },
                    MenuItem {
                        icon: html! {
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"></path>
                            </svg>
                        },
                        text: "Graphic Design",
                        link: "#graphic-design",
                        class: "",
                        title: "",
                        submenus: vec![],
                    },
                ],
            },
            MenuItem {
                icon: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"></path>
                    </svg>
                },
                text: "Contact",
                link: "#contact",
                class: "",
                title: "",
                submenus: vec![],
            },
        ],
        width_collapsed: "w-16",
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        width_expanded: "w-64",
        padding_collapsed: "p-2",
        padding_expanded: "p-4",
        display_collapsed: "hidden",
        display_expanded: "flex",
        justify_content: "flex-col",
        align_items: "items-start",
        height: "h-screen",
        background_color: "bg-gradient-to-r from-yellow-800 to-orange-700",
        font: "text-gray-800 text-2xl",
        icon_color: "yellow",
        button_border_radius: "rounded",
        button_background_color: "bg-blue-500",
        button_width: "w-full",
        button_height: "h-12",
        ..SidebarProps::default()
    };

    let sidebar_props2 = SidebarProps {
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
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
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

    let sidebar_props3 = SidebarProps {
        sider_collapsed: false,
        menu_items: vec![
            MenuItem {
                icon: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                    </svg>
                },
                text: "Users",
                link: "#users",
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
                text: "Roles",
                link: "#roles",
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
                text: "Settings",
                link: "#settings",
                class: "",
                title: "",
                submenus: vec![],
            },
        ],
        width_collapsed: "w-24",
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        width_expanded: "w-80",
        padding_collapsed: "p-3",
        padding_expanded: "p-5",
        display_collapsed: "hidden",
        display_expanded: "flex",
        justify_content: "flex-col",
        align_items: "items-center",
        height: "h-screen",
        background_color: "bg-purple-900",
        font: "text-gray-200",
        icon_color: "green",
        button_border_radius: "rounded-full",
        button_background_color: "bg-red-500",
        button_width: "w-3/4",
        button_height: "h-12",
        ..SidebarProps::default()
    };

    let sidebar_props4 = SidebarProps {
        sider_collapsed: false,
        title: "Projects",
        menu_items: vec![
            MenuItem {
                icon: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                    </svg>
                },
                text: "Active",
                link: "#active-projects",
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
                text: "Completed",
                link: "#completed-projects",
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
                text: "Archived",
                link: "#archived-projects",
                class: "",
                title: "",
                submenus: vec![],
            },
        ],
        width_collapsed: "w-16",
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        width_expanded: "w-56",
        padding_collapsed: "p-1",
        padding_expanded: "p-3",
        display_collapsed: "hidden",
        display_expanded: "flex",
        justify_content: "flex-col",
        align_items: "items-start",
        height: "h-screen",
        background_color: "bg-indigo-800",
        font: "text-white",
        icon_color: "yellow",
        button_border_radius: "rounded-lg",
        button_background_color: "bg-green-500",
        button_width: "w-full",
        button_height: "h-12",
        ..SidebarProps::default()
    };

    let sidebar_props5 = SidebarProps {
        sider_collapsed: false,
        title: "Admin Dashboard",
        menu_items: vec![
            MenuItem {
                icon: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 4v16m6-16v16m6-16v16"></path>
                    </svg>
                },
                text: "Dashboard",
                link: "#dashboard",
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
                text: "Users",
                link: "#users",
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
                text: "Settings",
                link: "#settings",
                class: "",
                title: "",
                submenus: vec![],
            },
        ],
        width_collapsed: "w-20",
        width_expanded: "w-72",
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        padding_collapsed: "p-2",
        padding_expanded: "p-4",
        display_collapsed: "hidden",
        display_expanded: "flex",
        justify_content: "flex-col",
        align_items: "items-center",
        height: "h-screen",
        background_color: "bg-blue-800",
        font: "text-white",
        icon_color: "yellow",
        button_border_radius: "rounded",
        button_background_color: "bg-yellow-500",
        button_width: "w-full",
        button_height: "h-12",
        ..SidebarProps::default()
    };

    let sidebar_props6 = SidebarProps {
        sider_collapsed: true,
        title: "Product Management",
        menu_items: vec![
            MenuItem {
                icon: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 4v16m6-16v16m6-16v16"></path>
                    </svg>
                },
                text: "Products",
                link: "#products",
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
                text: "Categories",
                link: "#categories",
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
                text: "Orders",
                link: "#orders",
                class: "",
                title: "",
                submenus: vec![],
            },
        ],
        width_collapsed: "w-16",
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        width_expanded: "w-60",
        padding_collapsed: "p-0",
        padding_expanded: "p-3",
        display_collapsed: "hidden",
        display_expanded: "flex",
        justify_content: "flex-col",
        align_items: "items-center",
        height: "h-screen",
        background_color: "bg-gray-900",
        font: "text-white",
        icon_color: "gray",
        button_border_radius: "rounded-none",
        button_background_color: "bg-green-500",
        button_width: "w-full",
        button_height: "h-12",
        ..SidebarProps::default()
    };
    html! {
        <div class="flex">
            <Sidebar ..sidebar_props />
            <Sidebar ..sidebar_props1 />
            <Sidebar ..sidebar_props2 />
            <Sidebar ..sidebar_props3 />
            <Sidebar ..sidebar_props4 />
            <Sidebar ..sidebar_props5 />
            <Sidebar ..sidebar_props6 />
        </div>
    }
}
