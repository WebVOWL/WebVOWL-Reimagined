use leptos::prelude::*;
use std::fmt::Display;

pub enum ToolTipDelay {
    None,
    Short,
    Medium,
    Long,
}

impl Display for ToolTipDelay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolTipDelay::None => write!(f, "delay-0"),

            ToolTipDelay::Short => write!(f, "delay-200"),

            ToolTipDelay::Medium => write!(f, "delay-500"),

            ToolTipDelay::Long => write!(f, "delay-1000"),
        }
    }
}

pub enum ToolTipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl ToolTipPosition {
    pub fn get_position(&self) -> String {
        match self {
            ToolTipPosition::Top => "bottom-full left-1/2 z-20 mb-3 -translate-x-1/2".to_string(),
            ToolTipPosition::Bottom => "left-1/2 top-full z-20 mt-3 -translate-x-1/2".to_string(),
            ToolTipPosition::Left => " right-full top-1/2 z-20 mr-3 -translate-y-1/2".to_string(),
            ToolTipPosition::Right => "left-full top-1/2 z-20 ml-3 -translate-y-1/2".to_string(),
        }
    }

    pub fn get_content_position(&self) -> String {
        match self {
            ToolTipPosition::Top => "-bottom-1 left-1/2 -z-10 h-2 w-2 -translate-x-1/2".to_string(),
            ToolTipPosition::Bottom => "-top-1 left-1/2 -z-10 h-2 w-2 -translate-x-1/2".to_string(),
            ToolTipPosition::Left => "-right-1 top-1/2 -z-10 h-2 w-2 -translate-y-1/2".to_string(),
            ToolTipPosition::Right => "-left-1 top-1/2 -z-10 h-2 w-2 -translate-y-1/2".to_string(),
        }
    }
}

#[component]
pub fn ToolTip<T>(
    /// The text to display in the tooltip.
    #[prop(into)]
    content: Signal<T>,
    /// The position of the tooltip.
    #[prop(default = ToolTipPosition::Top)]
    position: ToolTipPosition,
    /// The delay in milliseconds before the tooltip is shown.
    #[prop(default = ToolTipDelay::Medium)]
    delay: ToolTipDelay,
    children: Children,
) -> impl IntoView
where
    T: Display + Send + Sync + Clone + 'static,
{
    let tooltip_class = format!(
        "
        absolute \
        {} \
        whitespace-nowrap \
        rounded-[5px] \
        bg-white \
        px-3 \
        py-2 \
        text-sm \
        font-medium \
        text-dark \
        transition-opacity \
        {} \
        duration-150 \
        opacity-0 \
        shadow-lg \
        group-hover:opacity-100 \
        dark:bg-dark-2 \
        dark:text-white \
        dark:shadow-none
        ",
        position.get_position(),
        delay
    );
    let content_class = format!(
        "
        absolute \
        {} \
        rotate-45 \
        bg-white \
        dark:bg-dark-2
        ",
        position.get_content_position()
    );

    view! {
        <div class="group relative inline-block m-2">
            {children()}
            <div class=tooltip_class>
                <span class=content_class></span>
                {move || content.get().to_string()}
            </div>
        </div>
    }
}
