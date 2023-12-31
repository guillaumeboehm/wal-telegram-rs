use std::{collections::HashMap, rc::Rc, cell::RefCell};
use crate::color::Color;

pub fn get_telegram_colors(colors: &HashMap<String, Rc<RefCell<Color>>>) -> String {

    let elements: Rc<RefCell<HashMap<String, Option<Rc<RefCell<Color>>>>>> = Rc::new(RefCell::new(HashMap::new()));
    let ret: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

    let add_any = |el: &str, col: Option<Rc<RefCell<Color>>>| {
        let color = col.to_owned();

        if !col.is_none() {
            // If it is a Rc<Color>
            ret.as_ref().borrow_mut().push_str((String::from(el) + ": #" + color.as_ref().unwrap().borrow().to_hex().as_str() + ";\n").as_str());
        }
        else {
            // If comment
            ret.as_ref().borrow_mut().push_str((String::from("\n// ") + el + "\n").as_str());
        }

        elements.as_ref().borrow_mut().insert(String::from(el), color);
    };

    // Add a color with the pointer directly
    let add_color = |el: &str, col: Rc<RefCell<Color>>| {
        add_any(el, Some(col));
    };

    // Add a comment to the colors file
    let add_comment = |el: &str| {
        add_any(el, None);
    };

    // Get an already defined element's pointer
    let get_element = |el: &str| -> Rc<RefCell<Color>> {
        let col = elements.borrow().get(el)
                        .expect(format!("Could not find '{el}' in the previously defined elements").as_str())
                        .clone()
                        .expect("Color should not be none");

        return col;
    };

    add_comment("Color palette");
    let mut color_keys: Vec<_> = colors.keys().collect();
    color_keys.sort_by(|a, b| {
        let a_int: u32 = a.split_at(5).1.parse().unwrap();
        let b_int: u32 = b.split_at(5).1.parse().unwrap();
        a_int.cmp(&b_int)
    });

    for col_name in color_keys {
        add_color(format!("{col_name}").as_str(), colors[col_name].clone());
    }

    add_comment("Colors for testing purposes");
    add_color("colorPink", Rc::new(RefCell::new(Color::from_hex("ff7fc6").unwrap())));
    add_color("colorGreen", Rc::new(RefCell::new(Color::from_hex("0bd500").unwrap())));

    add_comment("Special common colors");
    add_color("colorError", Rc::new(RefCell::new(Color::from_hex("#d55070").unwrap())));
    add_color("colorSuccess", Rc::new(RefCell::new(Color::from_hex("#6bf576").unwrap())));

    add_comment("Basic window options");
    // \/ bg left menu list items + right click menu
    add_color("windowBg", get_element("color0").clone());
    // \/ fg plain text + right click menu options
    add_color("windowFg", get_element("windowBg").borrow().contrast(0));
    // \/ bg left menu list items and right click menu with cursor over
    add_color("windowBgOver", get_element("windowBg").borrow().contrast(13));
    // \/ bg left menu list items and right click menu with click pressed
    add_color("windowBgRipple", get_element("windowBgOver").borrow().darker(10));
    // \/ fg menu options right button with cursor over
    add_color("windowFgOver", get_element("windowBgOver").borrow().contrast(0));
    // \/ bottom left text in the left menu + faded text in the options menu
    add_color("windowSubTextFg", get_element("windowBg").borrow().contrast(4).borrow().alpha(50));
    // \/ [UNTESTED]: in theory like the option above but with cursor above ( Doesn't seem to work )
    add_color("windowSubTextFgOver", get_element("windowSubTextFg").borrow().darker(10));
    // \/ bold text color, found in the left menu options, option menu titles, and group description titles
    add_color("windowBoldFg", get_element("windowFg").borrow().darker(20));
    // \/ same as above but with the cursor above, for example in the left menu options
    add_color("windowBoldFgOver", get_element("windowFgOver").borrow().darker(10));
    // \/ Checkboxes background, Rounded + buttons, Emotes icon (foreground), Microphone icon, and other things that 'fill with colour', see options menu
    add_color("windowBgActive", get_element("windowBg").borrow().contrast(7));
    // \/ fg of elements using the bg above
    add_color("windowFgActive", get_element("windowBgActive").borrow().contrast(0));
    // \/ user active status text, input text hint if traveling up when active, options category
    // \/ titles, options state text (eg. for the language option -> English)
    add_color("windowActiveTextFg", get_element("color15").borrow().lighter(20));
    // \/ contour shadow left menu, options menu, right click menu and all the window elements with a shadow
    add_color("windowShadowFg", get_element("windowFg"));
    // \/ [UNTESTED]: fallback for shadows without opacity
    add_color("windowShadowFgFallback", get_element("windowFg"));

    add_comment("Shadow");
    // \/ most shadows (with opacity) (the color of the divider between parts of the home screen)
    add_color("shadowFg", get_element("windowBg").borrow().contrast(3).borrow().alpha(30));

    add_comment("Slide");
    // \/ Slide animation that appears when you press the arrow of a forwarded message (from chat to profile), or in the right drawer when looking at posted elements from the User Info menu.
    add_color("slideFadeOutBg", colors["color0"].borrow().alpha(10));
    // \/ [UNTESTED]: still talking about the animation explained above, it is the line on the right side that moves to the left
    add_color("slideFadeOutShadowFg", get_element("windowShadowFg"));

    add_comment("Image");
    // \/ [UNTESTED]: When the photo is smaller than the max. size
    add_color("imageBg", colors["color2"].clone());
    // \/ image background when it is an image with opacity, even if this is not required
    add_color("imageBgTransparent", colors["color7"].clone());

    add_comment("Active");
    // \/ bg color active button, eg. first button at the top left in the options menu, the "take a
    // photo" button in the profile, "Add" button in sticker packs selection
    add_color("activeButtonBg", colors["color2"].clone());
    // \/ as above but with the cursor above
    add_color("activeButtonBgOver", get_element("activeButtonBg").borrow().lighter(20));
    // \/ as above but ripple effect, click held
    add_color("activeButtonBgRipple", get_element("activeButtonBg").borrow().lighter(50));
    // \/ button text explained above
    add_color("activeButtonFg", get_element("activeButtonBg").borrow().contrast(0));
    // \/ Button text explained above but with cursor above
    add_color("activeButtonFgOver", get_element("activeButtonFg").borrow().lighter(20));
    // \/ when you select a message, the numbers next to forward and delete
    add_color("activeButtonSecondaryFg", get_element("activeButtonBg").borrow().contrast(2));
    // \/ same as above but with the cursor above
    add_color("activeButtonSecondaryFgOver", get_element("activeButtonSecondaryFg").borrow().lighter(20));
    // \/ Input text bottom line when focused, eg. line under the name choice when creating a group
    add_color("activeLineFg", colors["color2"].clone());
    // \/ same as above but when errors occur
    add_color("activeLineFgError", get_element("colorError"));

    // TODO: Reworked until here
    add_comment("Light");
    // \/ button on the right in the options menu and in short the light buttons
    add_color("lightButtonBg", colors["color0"].clone());
    // \/ same as above but with the cursor above
    add_color("lightButtonBgOver", colors["color0"].borrow().lighter(40));
    // \/ same as above but ripple effect, click held
    add_color("lightButtonBgRipple", colors["color0"].borrow().lighter(60));
    // \/ button text explained above
    add_color("lightButtonFg", colors["color2"].clone());
    // \/ Button text unfolded above with cursor above
    add_color("lightButtonFgOver", get_element("lightButtonFg"));

    add_comment("Attention");
    // \/ [UNTESTED]: default attention button text (like confirm button on log out)
    add_color("attentionButtonFg", colors["color1"].clone());
    // \/ [UNTESTED]: default attention button text with mouse over
    add_color("attentionButtonFgOver", colors["color1"].borrow().lighter(30));
    // \/ [UNTESTED]: default attention button background with mouse over
    add_color("attentionButtonBgOver", colors["color0"].borrow().lighter(40));
    // \/ [UNTESTED]: default attention button ripple effect
    add_color("attentionButtonBgRipple", colors["color0"].borrow().lighter(60));

    add_comment("Outline");
    // \/ [UNTESTED]: default left outlined button background (like shared media links in profiles)
    add_color("outlineButtonBg", get_element("windowBg"));
    // \/ [UNTESTED]: default left outlined button background with mouse over
    add_color("outlineButtonBgOver", colors["color0"].borrow().lighter(40));
    // \/ [UNTESTED]: default left outlined button left outline border
    add_color("outlineButtonOutlineFg", colors["color2"].clone());
    // \/ [UNTESTED]: default left outlined button ripple effect
    add_color("outlineButtonBgRipple", colors["color0"].borrow().lighter(60));

    add_comment("Menu");
    // \/ top and bottom lines of popup menus, such as the three dots at the top right in chats
    add_color("menuBg", colors["color0"].clone());
    // \/ [UNTESTED]: in theory the comments page of the popu menu with the cursor above
    add_color("menuBgOver", colors["color0"].borrow().lighter(40));
    // \/ [UNTESTED]: in theory the same as above but ripple effect
    add_color("menuBgRipple", colors["color0"].borrow().lighter(60));
    // \/ options menu icons and bar above in the chat area
    add_color("menuIconFg", colors["color7"].clone());
    // \/ same as above but with cursor above
    add_color("menuIconFgOver", colors["color7"].borrow().lighter(40));
    // \/ in the message field, if you press the right button, there is an arrow in the popup menu
    add_color("menuSubmenuArrowFg", colors["color7"].clone());
    // \/ text disabled in the popup menu (right click in the search field or in the text field)
    add_color("menuFgDisabled", colors["color7"].borrow().darker(40));
    // \/ separator in the menu right click in input field
    add_color("menuSeparatorFg", colors["color7"].borrow().darker(40));

    add_comment("Scroll");
    // \/ default scroll bar current rectangle, the bar itself (like in chats list)
    add_color("scrollBarBg", colors["color7"].borrow().alpha(30));
    // \/ default scroll bar current rectangle with mouse over it
    add_color("scrollBarBgOver", colors["color7"].borrow().alpha(45));
    // \/ default scroll bar background
    add_color("scrollBg", colors["color7"].borrow().alpha(05));
    // \/ default scroll bar background with mouse over the scroll bar
    add_color("scrollBgOver", colors["color7"].borrow().alpha(15));

    add_comment("Small");
    // \/ small cross for example next to the header in the emoji panel
    add_color("smallCloseIconFg", colors["color7"].borrow().darker(40));
    // \/ as above but with the cursor above
    add_color("smallCloseIconFgOver", colors["color7"].clone());

    add_comment("Radial");
    // \/ [UNTESTED]: default radial loader line (like in Media Viewer when loading a photo)
    add_color("radialFg", get_element("windowFgActive"));
    // \/ [UNTESTED]: default radial loader background (like in Media Viewer when loading a photo)
    add_color("radialBg", colors["color0"].borrow().alpha(30));

    add_comment("Placeholder");
    // \/ type the default placeholder text of the search bar and insert text
    add_color("placeholderFg", colors["color7"].clone());
    // \/ same as above, but when the field is in focus
    add_color("placeholderFgActive", colors["color7"].borrow().darker(40));

    add_comment("Input");
    // \/ like the alternative line fg when you are creating a channel
    add_color("inputBorderFg", colors["color7"].clone());

    add_comment("Filter");
    // \/ border that appears when you click in the search bar
    add_color("filterInputBorderFg", colors["color0"].borrow().lighter(40));
    // \/ bg search field inactive
    add_color("filterInputInactiveBg", colors["color8"].borrow().darker(30));
    // \/ bg active search field
    add_color("filterInputActiveBg", colors["color8"].borrow().darker(20));

    add_comment("Checkbox");
    // \/ emoji category icons and also un'ticked' tick boxes
    add_color("checkboxFg", colors["color7"].borrow().darker(40));

    add_comment("Slider");
    // \/ slider not active
    add_color("sliderBgInactive", colors["color7"].borrow().darker(40));
    // \/ active slider
    add_color("sliderBgActive", get_element("windowBgActive"));

    add_comment("Tooltip");
    // \/ bg of the tooltip field, like when you wait with the cursor over the timestamp of the mex
    add_color("tooltipBg", colors["color7"].clone());
    // \/ fg of the tooltip
    add_color("tooltipFg", colors["color0"].clone());
    // \/ tooltip edges
    add_color("tooltipBorderFg", colors["color7"].clone());

    add_comment("Title");
    // \/ [UNTESTED]: one pixel line shadow at the bottom of custom window title
    add_color("titleShadow", colors["color0"].borrow().alpha(05));
    // \/ [UNTESTED]: custom window title background when window is inactive
    add_color("titleBg", colors["color0"].clone());
    // \/ [UNTESTED]: custom window title background when window is active
    add_color("titleBgActive", get_element("titleBg"));
    // \/ [UNTESTED]: custom window title minimize/maximize/restore button background when window is inactive (Windows only)
    add_color("titleButtonBg", get_element("titleBg"));
    // \/ [UNTESTED]: custom window title minimize/maximize/restore button icon when window is inactive (Windows only)
    add_color("titleButtonFg", colors["color7"].clone());
    // \/ [UNTESTED]: custom window title minimize/maximize/restore button background with mouse over when window is inactive (Windows only)
    add_color("titleButtonBgOver", colors["color0"].borrow().lighter(40));
    // \/ [UNTESTED]: custom window title minimize/maximize/restore button icon with mouse over when window is inactive (Windows only)
    add_color("titleButtonFgOver", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: custom window title minimize/maximize/restore button background when window is active (Windows only)
    add_color("titleButtonBgActive", get_element("titleButtonBg"));
    // \/ [UNTESTED]: custom window title minimize/maximize/restore button icon when window is active (Windows only)
    add_color("titleButtonFgActive", get_element("titleButtonFg"));
    // \/ [UNTESTED]: custom window title minimize/maximize/restore button background with mouse over when window is active (Windows only)
    add_color("titleButtonBgActiveOver", get_element("titleButtonBgOver"));
    // \/ [UNTESTED]: custom window title minimize/maximize/restore button icon with mouse over when window is active (Windows only)
    add_color("titleButtonFgActiveOver", get_element("titleButtonFgOver"));
    // \/ [UNTESTED]: custom window title close button background when window is inactive (Windows only)
    add_color("titleButtonCloseBg", get_element("titleButtonBg"));
    // \/ [UNTESTED]: custom window title close button icon when window is inactive (Windows only)
    add_color("titleButtonCloseFg", get_element("titleButtonFg"));
    // \/ [UNTESTED]: custom window title close button background with mouse over when window is inactive (Windows only)
    add_color("titleButtonCloseBgOver", colors["color0"].borrow().lighter(40));
    // \/ [UNTESTED]: custom window title close button icon with mouse over when window is inactive (Windows only)
    add_color("titleButtonCloseFgOver", get_element("windowFgActive"));
    // \/ [UNTESTED]: custom window title close button background when window is active (Windows only)
    add_color("titleButtonCloseBgActive", get_element("titleButtonCloseBg"));
    // \/ [UNTESTED]: custom window title close button icon when window is active (Windows only)
    add_color("titleButtonCloseFgActive", get_element("titleButtonCloseFg"));
    // \/ [UNTESTED]: custom window title close button background with mouse over when window is active (Windows only)
    add_color("titleButtonCloseBgActiveOver", get_element("titleButtonCloseBgOver"));
    // \/ [UNTESTED]: custom window title close button icon with mouse over when window is active (Windows only)
    add_color("titleButtonCloseFgActiveOver", get_element("titleButtonCloseFgOver"));
    // \/ [UNTESTED]: custom window title text when window is inactive (macOS only)
    add_color("titleFg", colors["color7"].clone());
    // \/ [UNTESTED]: custom window title text when window is active (macOS only)
    add_color("titleFgActive", colors["color7"].borrow().lighter(40));

    add_comment("Tray");
    // \/ [UNTESTED]: tray icon counter background
    add_color("trayCounterBg", colors["color2"].clone());
    // \/ [UNTESTED]: tray icon counter background if all unread messages are muted
    add_color("trayCounterBgMute", colors["color0"].clone());
    // \/ [UNTESTED]: tray icon counter text
    add_color("trayCounterFg", colors["color7"].clone());
    // \/ [UNTESTED]: tray icon counter background when tray icon is pressed or when dark theme of macOS is used (macOS only)
    add_color("trayCounterBgMacInvert", colors["color7"].clone());
    // \/ [UNTESTED]: tray icon counter text when tray icon is pressed or when dark theme of macOS is used (macOS only)
    add_color("trayCounterFgMacInvert", colors["color2"].clone());

    add_comment("Layer");
    // \/ fade menu options and left menu
    add_color("layerBg", colors["color0"].borrow().alpha(45));

    add_comment("Cancel");
    // \/ fg cross to close the options menu and other things
    add_color("cancelIconFg", colors["color7"].borrow().darker(40));
    // \/ same as above but with cursor above the cross
    add_color("cancelIconFgOver", colors["color7"].clone());

    add_comment("Box");
    // \/ bg menu options
    add_color("boxBg", get_element("windowBg"));
    // \/ fg menu options
    add_color("boxTextFg", get_element("windowFg"));
    // \/ [UNTESTED]: accepted box text (like when choosing username that is not occupied)
    add_color("boxTextFgGood", colors["color2"].clone());
    // \/ [UNTESTED]: rejecting box text (like when choosing username that is occupied)
    add_color("boxTextFgError", colors["color1"].clone());
    // \/ box text, such as confirmation after changing theme
    add_color("boxTitleFg", colors["color7"].borrow().lighter(40));
    // \/ bg box search field, type search in the contacts option in the left menu
    add_color("boxSearchBg", colors["color0"].clone());
    // \/ subtext of the boxTitleFg, type where you can see the number of people to add when you are creating a group
    add_color("boxTitleAdditionalFg", colors["color7"].borrow().darker(40));
    // \/ other crosses
    add_color("boxTitleCloseFg", get_element("cancelIconFg"));
    // \/ other crosses with cursor above
    add_color("boxTitleCloseFgOver", get_element("cancelIconFgOver"));

    add_comment("Members");
    // \/ text when the member addition limit is exceeded (since it is very high, it's troublesome to check)
    add_color("membersAboutLimitFg", colors["color1"].clone());

    add_comment("Contacts");
    // \/ bg of the boxes that contain the contacts in the appropriate section accessible from the menu on the left
    add_color("contactsBg", colors["color0"].borrow().lighter(40));
    // \/ same as above but with the cursor above
    add_color("contactsBgOver", colors["color0"].clone());
    // \/ fg of the contact names in the section described above
    add_color("contactsNameFg", get_element("boxTextFg"));
    // \/ fg of the status of the contact names
    add_color("contactsStatusFg", colors["color7"].borrow().darker(40));
    // \/ same as above but with the cursor above
    add_color("contactsStatusFgOver", colors["color7"].borrow().darker(40));
    // \/ fg of the online writing in the online contacts precisely
    add_color("contactsStatusFgOnline", colors["color10"].clone());

    add_comment("Photo");
    // \/ the crop background of the chosen image (when you need to set an image for the group or for your profile)
    add_color("photoCropFadeBg", get_element("layerBg"));
    // \/ small rectangles that delimit the image you are setting
    add_color("photoCropPointFg", colors["color7"].borrow().alpha(45));

    add_comment("Call");
    // \/ [UNTESTED]: received phone call arrow (in calls list box)
    add_color("callArrowFg", colors["color2"].clone());
    // \/ [UNTESTED]: missed phone call arrow (in calls list box)
    add_color("callArrowMissedFg", colors["color1"].clone());

    add_comment("Intro");
    // \/ [UNTESTED]: login background
    add_color("introBg", get_element("windowBg"));
    // \/ [UNTESTED]: login title text
    add_color("introTitleFg", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: login description text
    add_color("introDescriptionFg", colors["color7"].clone());
    // \/ [UNTESTED]: login error text (like when providing a wrong log in code)
    add_color("introErrorFg", colors["color1"].clone());
    // \/ [UNTESTED]: intro gradient top (from)
    add_color("introCoverTopBg", colors["color2"].clone());
    // \/ [UNTESTED]: intro gradient bottom (to)
    add_color("introCoverBottomBg", colors["color2"].clone());
    // \/ [UNTESTED]: intro cloud graphics
    add_color("introCoverIconsFg", colors["color2"].borrow().lighter(40));
    // \/ [UNTESTED]: intro plane traces
    add_color("introCoverPlaneTrace", colors["color2"].borrow().lighter(40));
    // \/ [UNTESTED]: intro plane part
    add_color("introCoverPlaneInner", colors["color1"].borrow().lighter(40));
    // \/ [UNTESTED]: intro plane part
    add_color("introCoverPlaneOuter", colors["color1"].clone());
    // \/ [UNTESTED]: intro plane part
    add_color("introCoverPlaneTop", colors["color7"].borrow().lighter(40));

    add_comment("Dialogs default");
    // \/ main menu and lock telegram icon
    add_color("dialogsMenuIconFg", get_element("menuIconFg"));
    // \/ main menu and lock telegram icon with mouse over
    add_color("dialogsMenuIconFgOver", get_element("menuIconFgOver"));
    // \/ dialogue box bg
    add_color("dialogsBg", get_element("windowBg"));
    // \/ dialogue box fg names
    add_color("dialogsNameFg", colors["color7"].borrow().lighter(40));
    // \/ dialogue box group or contact icons
    add_color("dialogsChatIconFg", get_element("dialogsNameFg"));
    // \/ date text dialogue box
    add_color("dialogsDateFg", colors["color7"].borrow().darker(40));
    // \/ message text dialogue box (small under the name)
    add_color("dialogsTextFg", colors["color7"].clone());
    // \/ sender's message text dialogue box
    add_color("dialogsTextFgService", colors["color7"].clone());
    // \/ draft text color dialogue box
    add_color("dialogsDraftFg", colors["color1"].clone());
    // \/ bg verified profile icon
    add_color("dialogsVerifiedIconBg", colors["color10"].clone());
    // \/ fg verified profile icon
    add_color("dialogsVerifiedIconFg", colors["color0"].clone());
    // \/ send message icon (clock)
    add_color("dialogsSendingIconFg", colors["color10"].clone());
    // \/ single/double ticks to confirm sending message
    add_color("dialogsSentIconFg", colors["color10"].clone());
    // \/ [UNTESTED]: chat list unread badge background for not muted chat
    add_color("dialogsUnreadBg", colors["color1"].clone());
    // \/ fg pinned icon for pinned chats
    add_color("dialogsUnreadBgMuted", colors["color7"].borrow().darker(40));
    // \/ [UNTESTED]: chat list unread badge text
    add_color("dialogsUnreadFg", colors["color7"].borrow().lighter(40));

    add_comment("Dialogs over");
    // \/ cursor over dialog box
    add_color("dialogsBgOver", colors["color2"].borrow().darker(50));
    // \/ dialogue box fg names with cursor above
    add_color("dialogsNameFgOver", get_element("windowBoldFgOver"));
    // \/ dialogue box group or contact icons with cursor above
    add_color("dialogsChatIconFgOver", get_element("dialogsNameFgOver"));
    // \/ date text dialogue box with cursor above
    add_color("dialogsDateFgOver", colors["color7"].borrow().darker(40));
    // \/ message text dialogue box (small under the name) with cursor above
    add_color("dialogsTextFgOver", colors["color7"].clone());
    // \/ sender's message text dialogue box with cursor above
    add_color("dialogsTextFgServiceOver", colors["color7"].clone());
    // \/ draft text color dialogue box with cursor above
    add_color("dialogsDraftFgOver", get_element("dialogsDraftFg"));
    // \/ bg verified profile icon with cursor over
    add_color("dialogsVerifiedIconBgOver", colors["color2"].clone());
    // \/ fg verified profile icon with cursor over
    add_color("dialogsVerifiedIconFgOver", colors["color0"].clone());
    // \/ send message icon (clock) with cursor above
    add_color("dialogsSendingIconFgOver", get_element("dialogsSendingIconFg"));
    // \/ single/double ticks to confirm sending message with cursor above
    add_color("dialogsSentIconFgOver", colors["color10"].clone());
    // \/ [UNTESTED]: chat list unread badge background for not muted chat with mouse over
    add_color("dialogsUnreadBgOver", colors["color1"].borrow().darker(40));
    // \/ [UNTESTED]: chat list unread badge background for muted chat with mouse over
    add_color("dialogsUnreadBgMutedOver", colors["color7"].borrow().darker(40));
    // \/ [UNTESTED]: chat list unread badge text with mouse over
    add_color("dialogsUnreadFgOver", get_element("dialogsUnreadFg"));

    add_comment("Dialogs active");
    // \/ color bg dialog box active
    add_color("dialogsBgActive", colors["color2"].clone());
    // \/ dialog box fg names active
    add_color("dialogsNameFgActive", get_element("windowBoldFgOver"));
    // \/ dialog box active group or contact icons
    add_color("dialogsChatIconFgActive", get_element("dialogsNameFgActive"));
    // \/ active date text dialog box
    add_color("dialogsDateFgActive", colors["color7"].borrow().lighter(40));
    // \/ message text dialog box (small under the name) active
    add_color("dialogsTextFgActive", colors["color7"].borrow().lighter(40));
    // \/ sender message text dialog box active
    add_color("dialogsTextFgServiceActive", colors["color7"].borrow().lighter(40));
    // \/ active draft text color dialog box
    add_color("dialogsDraftFgActive", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: chat list verified icon background for current (active) chat
    add_color("dialogsVerifiedIconBgActive", get_element("dialogsTextFgActive"));
    // \/ [UNTESTED]: chat list verified icon check for current (active) chat
    add_color("dialogsVerifiedIconFgActive", get_element("dialogsBgActive"));
    // \/ message sending icon (clock) active
    add_color("dialogsSendingIconFgActive", colors["color7"].borrow().lighter(40));
    // \/ single/double ticks to confirm active message sending
    add_color("dialogsSentIconFgActive", get_element("dialogsTextFgActive"));
    // \/ [UNTESTED]: chat list unread badge background for not muted chat for current (active) chat
    add_color("dialogsUnreadBgActive", get_element("dialogsTextFgActive"));
    // \/ [UNTESTED]: chat list unread badge background for muted chat for current (active) chat
    add_color("dialogsUnreadBgMutedActive", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: chat list unread badge text for current (active) chat
    add_color("dialogsUnreadFgActive", colors["color7"].borrow().lighter(40));

    add_comment("Dialogs ripple");
    // \/ ripple effect on the dialogue box not active
    add_color("dialogsRippleBg", colors["color0"].borrow().lighter(60));
    // \/ ripple effect on the active dialogue box
    add_color("dialogsRippleBgActive", colors["color2"].borrow().lighter(40));

    add_comment("Dialogs forward");
    // \/ forwarding panel background (when forwarding messages in the smallest window size)
    add_color("dialogsForwardBg", get_element("dialogsBgActive"));
    // \/ forwarding panel text (when forwarding messages in the smallest window size)
    add_color("dialogsForwardFg", get_element("dialogsNameFgActive"));

    add_comment("Searched");
    // \/ bg of the text part when searching for messages in a single chat
    add_color("searchedBarBg", colors["color0"].borrow().lighter(40));
    // \/ fg of the text of the comment described above
    add_color("searchedBarFg", colors["color7"].clone());

    add_comment("Top");
    // \/ bg of the top bar on the right side of the main screen (inside the chats)
    add_color("topBarBg", colors["color0"].clone());

    add_comment("Emoji");
    // \/ bg of the emoji panel
    add_color("emojiPanBg", get_element("windowBg"));
    // \/ bg of the lower part of the emoji panel (categories)
    add_color("emojiPanCategories", colors["color0"].clone());
    // \/ fg descriptive header of the emoji panel
    add_color("emojiPanHeaderFg", colors["color7"].clone());
    // \/ [UNTESTED]: bg of the comment described above
    add_color("emojiPanHeaderBg", colors["color0"].clone());
    // \/ fg of the emoji not active (final part of the emoji panel)
    add_color("emojiIconFg", colors["color7"].clone());
    // \/ fg of the active emoji
    add_color("emojiIconFgActive", colors["color2"].clone());

    add_comment("Sticker");
    // \/ [UNTESTED]: delete X button background for custom sent stickers in stickers panel (legacy)
    add_color("stickerPanDeleteBg", colors["color0"].borrow().alpha(75));
    // \/ [UNTESTED]: delete X button icon for custom sent stickers in stickers panel (legacy)
    add_color("stickerPanDeleteFg", get_element("windowFgActive"));
    // \/ sticker and GIF preview background (when you press and hold on a sticker)
    add_color("stickerPreviewBg", colors["color0"].borrow().alpha(65));

    add_comment("History");
    // \/ inbox mex text not selected
    add_color("historyTextInFg", get_element("windowFg"));
    // \/ inbox mex text selected
    add_color("historyTextInFgSelected", colors["color7"].borrow().lighter(40));
    // \/ outbox mex text not selected
    add_color("historyTextOutFg", colors["color7"].clone());
    // \/ outbox mex text selected
    add_color("historyTextOutFgSelected", colors["color7"].borrow().lighter(40));
    // \/ inbox mex link text not selected
    add_color("historyLinkInFg", colors["color10"].clone());
    // \/ inbox text mex link selected
    add_color("historyLinkInFgSelected", colors["color7"].borrow().lighter(40));
    // \/ outbox mex link text not selected
    add_color("historyLinkOutFg", colors["color10"].clone());
    // \/ outbox mex link text selected
    add_color("historyLinkOutFgSelected", colors["color7"].borrow().lighter(40));
    // \/ inbox text filename media not selected
    add_color("historyFileNameInFg", get_element("historyTextInFg"));
    // \/ inbox text filename selected media
    add_color("historyFileNameInFgSelected", colors["color7"].borrow().lighter(40));
    // \/ outbox text filename media not selected
    add_color("historyFileNameOutFg", get_element("historyTextOutFg"));
    // \/ outbox text filename selected media
    add_color("historyFileNameOutFgSelected", colors["color7"].borrow().lighter(40));
    // \/ outbox tick/double tick unselected text
    add_color("historyOutIconFg", colors["color1"].borrow().lighter(70));
    // \/ outbox tick/double tick selected text
    add_color("historyOutIconFgSelected", colors["color7"].borrow().lighter(40));
    // \/ outbox tick/double tick average
    add_color("historyIconFgInverted", colors["color2"].clone());
    // \/ outbox send mex icon (clock)
    add_color("historySendingOutIconFg", colors["color2"].clone());
    // \/ inbox send message icon (clock)
    add_color("historySendingInIconFg", colors["color2"].clone());
    // \/ inbox media send icon (clock)
    add_color("historySendingInvertedIconFg", colors["color2"].borrow().alpha(75));
    // \/ [UNTESTED]: received phone call arrow
    add_color("historyCallArrowInFg", colors["color1"].clone());
    // \/ [UNTESTED]: received phone call arrow in a selected message
    add_color("historyCallArrowInFgSelected", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: missed phone call arrow
    add_color("historyCallArrowMissedInFg", get_element("callArrowMissedFg"));
    // \/ [UNTESTED]: missed phone call arrow in a selected message
    add_color("historyCallArrowMissedInFgSelected", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: outgoing phone call arrow
    add_color("historyCallArrowOutFg", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: outgoing phone call arrow
    add_color("historyCallArrowOutFgSelected", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: new unread messages bar background
    add_color("historyUnreadBarBg", colors["color0"].clone());
    // \/ [UNTESTED]: new unread messages bar shadow
    add_color("historyUnreadBarBorder", get_element("shadowFg"));
    // \/ [UNTESTED]: new unread messages bar text
    add_color("historyUnreadBarFg", colors["color1"].clone());
    // \/ [UNTESTED]: forwarding messages in a large window size 'choose recipient' background
    add_color("historyForwardChooseBg", colors["color0"].borrow().alpha(25));
    // \/ [UNTESTED]: forwarding messages in a large window size 'choose recipient' text
    add_color("historyForwardChooseFg", get_element("windowFgActive"));
    // \/ username 1 mex not selected
    add_color("historyPeer1NameFg", colors["color1"].clone());
    // \/ username 1 mex selected
    add_color("historyPeer1NameFgSelected", colors["color7"].borrow().lighter(40));
    // \/ bg userpic 1
    add_color("historyPeer1UserpicBg", colors["color1"].clone());
    // \/ 2 mex username not selected
    add_color("historyPeer2NameFg", colors["color2"].clone());
    // \/ 2 mex username selected
    add_color("historyPeer2NameFgSelected", colors["color7"].borrow().lighter(40));
    // \/ bg userpic 2
    add_color("historyPeer2UserpicBg", colors["color2"].clone());
    // \/ 3 mex username not selected
    add_color("historyPeer3NameFg", colors["color3"].clone());
    // \/ 3 mex username selected
    add_color("historyPeer3NameFgSelected", colors["color7"].borrow().lighter(40));
    // \/ bg userpic 3
    add_color("historyPeer3UserpicBg", colors["color3"].clone());
    // \/ 4 mex username not selected
    add_color("historyPeer4NameFg", colors["color4"].clone());
    // \/ 4 mex username selected
    add_color("historyPeer4NameFgSelected", colors["color7"].borrow().lighter(40));
    // \/ bg userpic 4
    add_color("historyPeer4UserpicBg", colors["color4"].clone());
    // \/ 5 mex username not selected
    add_color("historyPeer5NameFg", colors["color5"].clone());
    // \/ 5 mex username selected
    add_color("historyPeer5NameFgSelected", colors["color7"].borrow().lighter(40));
    // \/ bg userpic 5
    add_color("historyPeer5UserpicBg", colors["color5"].clone());
    // \/ 6 mex username not selected
    add_color("historyPeer6NameFg", colors["color6"].clone());
    // \/ 6 mex username selected
    add_color("historyPeer6NameFgSelected", colors["color7"].borrow().lighter(40));
    // \/ bg userpic 6
    add_color("historyPeer6UserpicBg", colors["color6"].clone());
    // \/ 7 mex username not selected
    add_color("historyPeer7NameFg", colors["color7"].clone());
    // \/ 7 mex username selected
    add_color("historyPeer7NameFgSelected", colors["color7"].borrow().lighter(40));
    // \/ bg userpic 7
    add_color("historyPeer7UserpicBg", colors["color7"].clone());
    // \/ 8 mex username not selected
    add_color("historyPeer8NameFg", colors["color8"].clone());
    // \/ 8 mex username selected
    add_color("historyPeer8NameFgSelected", colors["color7"].borrow().lighter(40));
    // \/ bg userpic 8
    add_color("historyPeer8UserpicBg", colors["color8"].clone());
    // \/ userpic initial fg
    add_color("historyPeerUserpicFg", get_element("windowFgActive"));
    // \/ bg normal slash container
    add_color("historyScrollBarBg", colors["color7"].borrow().alpha(45));
    // \/ bg slash container with cursor over
    add_color("historyScrollBarBgOver", colors["color7"].borrow().alpha(65));
    // \/ bg normal slash
    add_color("historyScrollBg", colors["color7"].borrow().alpha(25));
    // \/ bg bar with cursor above
    add_color("historyScrollBgOver", colors["color7"].borrow().alpha(40));

    add_comment("Msg");
    // \/ inbox mex bg not selected
    add_color("msgInBg", colors["color7"].borrow().darker(70));
    // \/ inbox mex bg selected
    add_color("msgInBgSelected", colors["color2"].clone());
    // \/ outbox mex bg not selected
    add_color("msgOutBg", colors["color8"].borrow().darker(60));
    // \/ outbox mex bg selected
    add_color("msgOutBgSelected", colors["color2"].clone());
    // \/ overlay over the selected message
    add_color("msgSelectOverlay", colors["color2"].borrow().alpha(25));
    // \/ overlay over the selected sticker
    add_color("msgStickerOverlay", colors["color2"].borrow().alpha(45));
    // \/ inbox text color information type forwarded by... not selected
    add_color("msgInServiceFg", get_element("windowActiveTextFg"));
    // \/ inbox text color information type forwarded by... selected
    add_color("msgInServiceFgSelected", colors["color7"].borrow().lighter(40));
    // \/ outbox text color information type submitted by... not selected
    add_color("msgOutServiceFg", colors["color10"].clone());
    // \/ outbox text color information type forwarded by... selected
    add_color("msgOutServiceFgSelected", colors["color7"].borrow().lighter(40));
    // \/ inbox ombre mex not selected
    add_color("msgInShadow", colors["color0"].borrow().alpha(00));
    // \/ inbox shadow mex selected
    add_color("msgInShadowSelected", colors["color2"].borrow().alpha(00));
    // \/ outbox shadow mex not selected
    add_color("msgOutShadow", colors["color0"].borrow().alpha(00));
    // \/ outbox shadow mex selected
    add_color("msgOutShadowSelected", colors["color2"].borrow().alpha(00));
    // \/ inbox hours sending message not selected
    add_color("msgInDateFg", colors["color7"].borrow().darker(40));
    // \/ inbox hours sending message selected
    add_color("msgInDateFgSelected", colors["color7"].borrow().lighter(40));
    // \/ outbox hours send mex not selected
    add_color("msgOutDateFg", colors["color7"].borrow().darker(40));
    // \/ outbox hours sending message selected
    add_color("msgOutDateFgSelected", colors["color7"].borrow().lighter(40));
    // \/ service fg mex (data mex type, group title changed, etc.)
    add_color("msgServiceFg", get_element("windowFgActive"));
    // \/ bg mex service not selected
    add_color("msgServiceBg", colors["color0"].clone());
    // \/ bg mex of selected service
    add_color("msgServiceBgSelected", colors["color10"].clone());
    // \/ inbox text color type forwarded from etc... not selected
    add_color("msgInReplyBarColor", colors["color10"].clone());
    // \/ inbox text color type forwarded from etc.... selected
    add_color("msgInReplyBarSelColor", colors["color7"].borrow().lighter(40));
    // \/ outbox text color type forwarded by etc.... not selected
    add_color("msgOutReplyBarColor", colors["color10"].clone());
    // \/ outbox text color type forwarded from etc.... selected
    add_color("msgOutReplyBarSelColor", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: Forwarded text color when dealing with images
    add_color("msgImgReplyBarColor", get_element("msgServiceFg"));
    // \/ inbox mex monospace not selected
    add_color("msgInMonoFg", colors["color7"].clone());
    // \/ inbox mex monospace selected
    add_color("msgInMonoFgSelected", colors["color7"].borrow().lighter(40));
    // \/ outbox mex monospace not selected
    add_color("msgOutMonoFg", colors["color7"].clone());
    // \/ outbox mex monospace selected
    add_color("msgOutMonoFgSelected", colors["color7"].borrow().lighter(40));
    // \/ mex media fg bubble hours sending
    add_color("msgDateImgFg", get_element("msgServiceFg"));
    // \/ mex media bg bubble hours sending
    add_color("msgDateImgBg", colors["color0"].borrow().alpha(30));
    // \/ mex media bg bubble hours sending with cursor over
    add_color("msgDateImgBgOver", colors["color0"].borrow().alpha(45));
    // \/ mex media bg bubble hours sending selected
    add_color("msgDateImgBgSelected", colors["color2"].borrow().alpha(50));
    // \/ inbox file media file mex download not selected
    add_color("msgFileThumbLinkInFg", get_element("lightButtonFg"));
    // \/ inbox file media file mex download selected
    add_color("msgFileThumbLinkInFgSelected", get_element("lightButtonFgOver"));
    // \/ outbox file media file mex download not selected
    add_color("msgFileThumbLinkOutFg", colors["color10"].clone());
    // \/ outbox file media file mex download selected
    add_color("msgFileThumbLinkOutFgSelected", colors["color7"].borrow().lighter(40));
    // \/ inbox bg audio file circle download
    add_color("msgFileInBg", colors["color2"].clone());
    // \/ inbox bg audio file download circle with cursor above
    add_color("msgFileInBgOver", colors["color2"].borrow().lighter(30));
    // \/ inbox bg audio file circle download selected
    add_color("msgFileInBgSelected", colors["color2"].borrow().lighter(50));
    // \/ outbox bg audio file circle download
    add_color("msgFileOutBg", colors["color2"].clone());
    // \/ outbox bg audio file circle download with cursor above
    add_color("msgFileOutBgOver", colors["color2"].borrow().lighter(30));
    // \/ outbox bg audio file circle download selected
    add_color("msgFileOutBgSelected", colors["color2"].borrow().lighter(50));
    // \/ [UNTESTED]: blue shared links / files without image square thumbnail
    add_color("msgFile1Bg", colors["color1"].clone());
    // \/ [UNTESTED]: blue shared files without image download circle background
    add_color("msgFile1BgDark", colors["color1"].borrow().darker(30));
    // \/ [UNTESTED]: blue shared files without image download circle background with mouse over
    add_color("msgFile1BgOver", colors["color1"].borrow().lighter(40));
    // \/ [UNTESTED]: blue shared files without image download circle background if file is selected
    add_color("msgFile1BgSelected", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: green shared links / shared files without image square thumbnail
    add_color("msgFile2Bg", colors["color2"].clone());
    // \/ [UNTESTED]: green shared files without image download circle background
    add_color("msgFile2BgDark", colors["color2"].borrow().darker(30));
    // \/ [UNTESTED]: green shared files without image download circle background with mouse over
    add_color("msgFile2BgOver", colors["color2"].borrow().lighter(40));
    // \/ [UNTESTED]: green shared files without image download circle background if file is selected
    add_color("msgFile2BgSelected", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: red shared links / shared files without image square thumbnail
    add_color("msgFile3Bg", colors["color3"].clone());
    // \/ [UNTESTED]: red shared files without image download circle background
    add_color("msgFile3BgDark", colors["color7"].borrow().darker(30));
    // \/ [UNTESTED]: red shared files without image download circle background with mouse over
    add_color("msgFile3BgOver", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: red shared files without image download circle background if file is selected
    add_color("msgFile3BgSelected", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: yellow shared links / shared files without image square thumbnail
    add_color("msgFile4Bg", colors["color3"].clone());

    // \/ FIXME: Seems to be gone  [UNTESTED]: yellow shared files without image download circle background"
    add_color("msgFile4BgDark", colors["color3"].borrow().darker(30));
    // \/ FIXME: Seems to be gone  [UNTESTED]: yellow shared files without image download circle background with mouse over"),
    add_color("msgFile4BgOver", colors["color3"].borrow().lighter(40));

    // \/ [UNTESTED]: yellow shared files without image download circle background if file is selected
    add_color("msgFile4BgSelected", colors["color7"].borrow().lighter(40));
    // \/ inbox ondina audio inactive unselected
    add_color("msgWaveformInActive", get_element("windowBgActive"));
    // \/ inbox wave audio inactive selected
    add_color("msgWaveformInActiveSelected", colors["color7"].borrow().lighter(40));
    // \/ inbox wave active audio not selected
    add_color("msgWaveformInInactive", colors["color7"].borrow().darker(30));
    // \/ inbox wave active audio selected
    add_color("msgWaveformInInactiveSelected", colors["color2"].borrow().lighter(40));
    // \/ outbox ondina audio inactive unchecked
    add_color("msgWaveformOutActive", colors["color2"].clone());
    // \/ outbox wave audio inactive selected
    add_color("msgWaveformOutActiveSelected", colors["color7"].borrow().lighter(40));
    // \/ outbox wave active audio not selected
    add_color("msgWaveformOutInactive", colors["color7"].borrow().darker(30));
    // \/ outbox wave active audio selected
    add_color("msgWaveformOutInactiveSelected", colors["color2"].borrow().lighter(40));
    // \/ [UNTESTED]: this is painted over a bot inline keyboard button (which has msgServiceBg background) when mouse is over that button
    add_color("msgBotKbOverBgAdd", colors["color7"].borrow().alpha(05));
    // \/ [UNTESTED]: bot inline keyboard button icon in the top-right corner (like in @vote bot when a poll is ready to be shared)
    add_color("msgBotKbIconFg", get_element("msgServiceFg"));
    // \/ [UNTESTED]: bot inline keyboard button ripple effect
    add_color("msgBotKbRippleBg", colors["color1"].borrow().alpha(05));

    add_comment("Download animations");
    // \/ inbox file download arrow not selected
    add_color("historyFileInIconFg", colors["color0"].clone());
    // \/ inbox download arrow selected file
    add_color("historyFileInIconFgSelected", colors["color10"].clone());
    // \/ inbox particle animation download file not selected
    add_color("historyFileInRadialFg", colors["color0"].clone());
    // \/ inbox particle animation download selected file
    add_color("historyFileInRadialFgSelected", get_element("historyFileInIconFgSelected"));
    // \/ outbox file download arrow not selected
    add_color("historyFileOutIconFg", colors["color0"].clone());
    // \/ outbox download arrow selected file
    add_color("historyFileOutIconFgSelected", colors["color10"].clone());
    // \/ outbox particle animation file download not selected
    add_color("historyFileOutRadialFg", get_element("historyFileOutIconFg"));
    // \/ outbox particle animation download selected file
    add_color("historyFileOutRadialFgSelected", colors["color10"].clone());
    // \/ fg photo/video download arrow not selected
    add_color("historyFileThumbIconFg", colors["color7"].borrow().lighter(40));
    // \/ fg photo/video download arrow selected
    add_color("historyFileThumbIconFgSelected", colors["color7"].borrow().lighter(40));
    // \/ fg particle animation photo/video download not selected
    add_color("historyFileThumbRadialFg", get_element("historyFileThumbIconFg"));
    // \/ fg particle animation download selected photo/video
    add_color("historyFileThumbRadialFgSelected", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: radial playback progress in round video messages
    add_color("historyVideoMessageProgressFg", get_element("historyFileThumbIconFg"));

    add_comment("YouTube");
    // \/ [UNTESTED]: youtube play icon background (when a link to a youtube video with a webpage preview is sent)
    add_color("youtubePlayIconBg", Color::from_hex("83131c").unwrap().alpha(50));
    // \/ [UNTESTED]: youtube play icon arrow (when a link to a youtube video with a webpage preview is sent)
    add_color("youtubePlayIconFg", get_element("windowFgActive"));

    add_comment("Video");
    // \/ [UNTESTED]: other video play icon background (like when a link to a vimeo video with a webpage preview is sent)
    add_color("videoPlayIconBg", colors["color0"].borrow().alpha(45));
    // \/ [UNTESTED]: other video play icon arrow (like when a link to a vimeo video with a webpage preview is sent)
    add_color("videoPlayIconFg", colors["color7"].borrow().lighter(40));

    add_comment("Toast");
    // \/ [UNTESTED]: toast notification background (like when you click on your t.me link when editing your username)
    add_color("toastBg", colors["color0"].borrow().alpha(65));
    // \/ [UNTESTED]: toast notification text (like when you click on your t.me link when editing your username)
    add_color("toastFg", get_element("windowFgActive"));

    add_comment("Report");
    // \/ [UNTESTED]: report spam panel background (like a non contact user writes your for the first time)
    add_color("reportSpamBg", colors["color0"].clone());
    // \/ [UNTESTED]: report spam panel text (when you send a report from that panel)
    add_color("reportSpamFg", get_element("windowFg"));

    add_comment("Composition area");
    // \/ bg arrow button to scroll to the bottom of the chat
    add_color("historyToDownBg", colors["color0"].clone());
    // \/ bg arrow button to scroll to the bottom of the chat with cursor above
    add_color("historyToDownBgOver", colors["color0"].borrow().lighter(40));
    // \/ bg arrow button to scroll to the bottom of the selected chat
    add_color("historyToDownBgRipple", colors["color0"].borrow().lighter(60));
    // \/ fg arrow button to scroll to the bottom of the chat
    add_color("historyToDownFg", colors["color7"].clone());
    // \/ fg arrow button to scroll to the bottom of the chat with cursor above
    add_color("historyToDownFgOver", get_element("menuIconFgOver"));
    // \/ button shadow
    add_color("historyToDownShadow", colors["color0"].borrow().alpha(25));
    // \/ bg composition area at the bottom right of the home screen
    add_color("historyComposeAreaBg", colors["color0"].clone());
    // \/ fg of the area just mentioned
    add_color("historyComposeAreaFg", get_element("historyTextInFg"));
    // \/ mex text selected in the composition area
    add_color("historyComposeAreaFgService", get_element("msgInDateFg"));
    // \/ fg composition area icons
    add_color("historyComposeIconFg", get_element("menuIconFg"));
    // \/ fg composition area icons with cursor above
    add_color("historyComposeIconFgOver", get_element("menuIconFgOver"));
    // \/ fg send message icon
    add_color("historySendIconFg", get_element("windowBgActive"));
    // \/ fg message sending icon with cursor above
    add_color("historySendIconFgOver", get_element("windowBgActive"));
    // \/ [UNTESTED]: pinned message area background
    add_color("historyPinnedBg", get_element("historyComposeAreaBg"));
    // \/ bg area reply, forward, edit mex
    add_color("historyReplyBg", get_element("historyComposeAreaBg"));
    // \/ fg left arrow icon in reply, forward, edit message area
    add_color("historyReplyIconFg", get_element("windowBgActive"));
    // \/ fg cross icon in reply, forward, edit message area
    add_color("historyReplyCancelFg", get_element("cancelIconFg"));
    // \/ fg cross icon in reply, forward, edit message area with cursor above
    add_color("historyReplyCancelFgOver", get_element("cancelIconFgOver"));
    // \/ [UNTESTED]: unblock / join channel / mute channel button background
    add_color("historyComposeButtonBg", get_element("historyComposeAreaBg"));
    // \/ [UNTESTED]: unblock / join channel / mute channel button background with mouse over
    add_color("historyComposeButtonBgOver", colors["color0"].borrow().lighter(40));
    // \/ [UNTESTED]: unblock / join channel / mute channel button ripple effect
    add_color("historyComposeButtonBgRipple", colors["color0"].borrow().lighter(60));

    add_comment("Overview");
    // \/ [UNTESTED]: shared files / links checkbox background for not selected rows when some rows are selected
    add_color("overviewCheckBg", colors["color0"].borrow().alpha(25));
    // \/ [UNTESTED]: shared files / links checkbox icon for not selected rows when some rows are selected
    add_color("overviewCheckFg", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: shared files / links checkbox icon for selected rows
    add_color("overviewCheckFgActive", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: shared photos / videos / links fill for selected rows
    add_color("overviewPhotoSelectOverlay", colors["color1"].borrow().alpha(10));

    add_comment("Profile");
    // \/ [UNTESTED]: group members list in group profile user last seen text with mouse over
    add_color("profileStatusFgOver", colors["color1"].clone());
    // \/ [UNTESTED]: profile verified check icon background
    add_color("profileVerifiedCheckBg", get_element("windowBgActive"));
    // \/ [UNTESTED]: profile verified check icon tick
    add_color("profileVerifiedCheckFg", get_element("windowFgActive"));
    // \/ [UNTESTED]: group members list admin star icon
    add_color("profileAdminStartFg", get_element("windowBgActive"));

    add_comment("Notifications");
    // \/ [UNTESTED]: custom notifications settings box monitor color
    add_color("notificationsBoxMonitorFg", get_element("windowFg"));
    // \/ [UNTESTED]: #6389a8, // custom notifications settings box monitor screen background
    add_color("notificationsBoxScreenBg", get_element("dialogsBgActive"));
    // \/ [UNTESTED]: custom notifications settings box small sample userpic placeholder
    add_color("notificationSampleUserpicFg", get_element("windowBgActive"));
    // \/ [UNTESTED]: custom notifications settings box small sample close button placeholder
    add_color("notificationSampleCloseFg", colors["color7"].clone());
    // \/ [UNTESTED]: custom notifications settings box small sample text placeholder
    add_color("notificationSampleTextFg", colors["color7"].clone());
    // \/ [UNTESTED]: custom notifications settings box small sample name placeholder
    add_color("notificationSampleNameFg", colors["color0"].borrow().lighter(40));

    add_comment("Change");
    // \/ [UNTESTED]: change phone number box left simcard icon
    add_color("changePhoneSimcardFrom", get_element("notificationSampleTextFg"));
    // \/ [UNTESTED]: change phone number box right simcard and plane icons
    add_color("changePhoneSimcardTo", get_element("notificationSampleNameFg"));

    add_comment("Main");
    // \/ bg menu on the left
    add_color("mainMenuBg", get_element("windowBg"));
    // \/ bg top cover menu on the left (top part)
    add_color("mainMenuCoverBg", colors["color2"].clone());
    // \/ fg top cover menu on the left
    add_color("mainMenuCoverFg", get_element("windowFgActive"));
    // \/ fg speech bubble icon in the left menu
    add_color("mainMenuCloudFg", colors["color7"].borrow().lighter(40));
    // \/ bg speech bubble icon in the left menu
    add_color("mainMenuCloudBg", colors["color4"].clone());

    add_comment("Media");
    // \/ inbox status text (type weight of the audio file) not selected
    add_color("mediaInFg", get_element("msgInDateFg"));
    // \/ inbox status text (type weight of the audio file) selected
    add_color("mediaInFgSelected", get_element("msgInDateFgSelected"));
    // \/ status text outbox (type weight of the audio file) not selected
    add_color("mediaOutFg", get_element("msgOutDateFg"));
    // \/ status text outbox (type weight of the audio file) selected
    add_color("mediaOutFgSelected", get_element("msgOutDateFgSelected"));
    // \/ [UNTESTED]: audio file player background
    add_color("mediaPlayerBg", get_element("windowBg"));
    // \/ [UNTESTED]: audio file player playback progress already played part
    add_color("mediaPlayerActiveFg", get_element("windowBgActive"));
    // \/ [UNTESTED]: audio file player playback progress upcoming (not played yet) part with mouse over
    add_color("mediaPlayerInactiveFg", get_element("sliderBgInactive"));
    // \/ [UNTESTED]: audio file player loading progress (when you're playing an audio file and switch to the previous one which is not loaded yet)
    add_color("mediaPlayerDisabledFg", colors["color1"].clone());

    add_comment("Mediaview");
    // \/ [UNTESTED]: file rectangle background (when you view a png file in Media Viewer and go to a previous, not loaded yet, file)
    add_color("mediaviewFileBg", get_element("windowBg"));
    // \/ [UNTESTED]: file name in file rectangle
    add_color("mediaviewFileNameFg", get_element("windowFg"));
    // \/ [UNTESTED]: file size text in file rectangle
    add_color("mediaviewFileSizeFg", get_element("windowSubTextFg"));
    // \/ [UNTESTED]: red file thumbnail placeholder corner in file rectangle (for a file without thumbnail, like .pdf)
    add_color("mediaviewFileRedCornerFg", colors["color1"].clone());
    // \/ [UNTESTED]: yellow file thumbnail placeholder corner in file rectangle (for a file without thumbnail, like .zip)
    add_color("mediaviewFileYellowCornerFg", colors["color2"].clone());
    // \/ [UNTESTED]: green file thumbnail placeholder corner in file rectangle (for a file without thumbnail, like .exe)
    add_color("mediaviewFileGreenCornerFg", colors["color3"].clone());
    // \/ [UNTESTED]: blue file thumbnail placeholder corner in file rectangle (for a file without thumbnail, like .dmg)
    add_color("mediaviewFileBlueCornerFg", colors["color4"].clone());
    // \/ [UNTESTED]: file extension text in file thumbnail placeholder in file rectangle
    add_color("mediaviewFileExtFg", get_element("activeButtonFg"));
    // \/ [UNTESTED]: context menu in Media Viewer background
    add_color("mediaviewMenuBg", colors["color0"].clone());
    // \/ [UNTESTED]: context menu item background with mouse over
    add_color("mediaviewMenuBgOver", colors["color0"].borrow().lighter(40));
    // \/ [UNTESTED]: context menu item ripple effect
    add_color("mediaviewMenuBgRipple", colors["color0"].borrow().lighter(60));
    // \/ [UNTESTED]: context menu item text
    add_color("mediaviewMenuFg", get_element("windowFgActive"));
    // \/ [UNTESTED]: media viewer background
    add_color("mediaviewBg", colors["color0"].borrow().darker(30));
    // \/ [UNTESTED]: media viewer background when viewing a video in full screen
    add_color("mediaviewVideoBg", get_element("imageBg"));
    // \/ [UNTESTED]: controls background (like next photo / previous photo)
    add_color("mediaviewControlBg", colors["color0"].borrow().darker(50));
    // \/ [UNTESTED]: controls icon (like next photo / previous photo)
    add_color("mediaviewControlFg", get_element("windowFgActive"));
    // \/ [UNTESTED]: caption text background (when viewing photo with caption)
    add_color("mediaviewCaptionBg", colors["color0"].borrow().darker(50));
    // \/ [UNTESTED]: caption text
    add_color("mediaviewCaptionFg", get_element("mediaviewControlFg"));
    // \/ [UNTESTED]: caption text link
    add_color("mediaviewTextLinkFg", colors["color7"].clone());
    // \/ [UNTESTED]: save to file toast message background in Media Viewer
    add_color("mediaviewSaveMsgBg", get_element("toastBg"));
    // \/ [UNTESTED]: save to file toast message text
    add_color("mediaviewSaveMsgFg", get_element("toastFg"));
    // \/ [UNTESTED]: video playback progress already played part
    add_color("mediaviewPlaybackActive", colors["color7"].clone());
    // \/ [UNTESTED]: video playback progress upcoming (not played yet) part
    add_color("mediaviewPlaybackInactive", colors["color7"].borrow().darker(50));
    // \/ [UNTESTED]: video playback progress already played part with mouse over
    add_color("mediaviewPlaybackActiveOver", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: video playback progress upcoming (not played yet) part with mouse over
    add_color("mediaviewPlaybackInactiveOver", colors["color7"].borrow().darker(30));
    // \/ [UNTESTED]: video playback progress text
    add_color("mediaviewPlaybackProgressFg", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: video playback controls icon
    add_color("mediaviewPlaybackIconFg", get_element("mediaviewPlaybackActive"));
    // \/ [UNTESTED]: video playback controls icon with mouse over
    add_color("mediaviewPlaybackIconFgOver", get_element("mediaviewPlaybackActiveOver"));
    // \/ [UNTESTED]: transparent filling part (when viewing a transparent .png file in Media Viewer)
    add_color("mediaviewTransparentBg", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: another transparent filling part
    add_color("mediaviewTransparentFg", colors["color7"].clone());
    // \/ [UNTESTED]: custom notification window background
    add_color("notificationBg", get_element("windowBg"));

    add_comment("Call");
    // \/ [UNTESTED]: phone call popup background
    add_color("callBg", colors["color0"].clone());
    // \/ [UNTESTED]: phone call popup name text
    add_color("callNameFg", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: phone call popup emoji fingerprint background
    add_color("callFingerprintBg", colors["color0"].borrow().alpha(40));
    // \/ [UNTESTED]: phone call popup status text
    add_color("callStatusFg", colors["color7"].clone());
    // \/ [UNTESTED]: phone call popup answer, hangup and mute mic icon
    add_color("callIconFg", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: phone call popup answer button background
    add_color("callAnswerBg", colors["color2"].clone());
    // \/ [UNTESTED]: phone call popup answer button ripple effect
    add_color("callAnswerRipple", colors["color2"].borrow().darker(30));
    // \/ [UNTESTED]: phone call popup answer button outer ripple effect
    add_color("callAnswerBgOuter", colors["color2"].borrow().lighter(30));
    // \/ [UNTESTED]: phone call popup hangup button background
    add_color("callHangupBg", colors["color1"].clone());
    // \/ [UNTESTED]: phone call popup hangup button ripple effect
    add_color("callHangupRipple", colors["color1"].borrow().darker(30));
    // \/ [UNTESTED]: phone call popup line busy cancel button background
    add_color("callCancelBg", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: phone call popup line busy cancel button icon
    add_color("callCancelFg", colors["color7"].borrow().darker(40));
    // \/ [UNTESTED]: phone call popup line busy cancel button ripple effect
    add_color("callCancelRipple", colors["color7"].borrow().lighter(40));
    // \/ [UNTESTED]: youtube play icon background (when a link to a youtube video with a webpage preview is sent)
    add_color("youtubePlayIconBg", Color::from_hex("83131c").unwrap().alpha(50));
    // \/ [UNTESTED]: phone call popup mute mic ripple effect
    add_color("callMuteRipple", Color::from_hex("ffffff").unwrap().alpha(05));
    // \/ [UNTESTED]: active phone call bar background
    add_color("callBarBg", get_element("dialogsBgActive"));
    // \/ [UNTESTED]: active phone call bar mute and hangup button ripple effect
    add_color("callBarMuteRipple", get_element("dialogsRippleBgActive"));
    // \/ [UNTESTED]: phone call bar with muted mic background
    add_color("callBarBgMuted", colors["color0"].borrow().lighter(40));
    // \/ [UNTESTED]: phone call bar with muted mic mute and hangup button ripple effect
    add_color("callBarUnmuteRipple", colors["color0"].borrow().lighter(40));
    // \/ [UNTESTED]: phone call bar text and icons
    add_color("callBarFg", get_element("dialogsNameFgActive"));

    add_comment("Important");
    // \/ [UNTESTED]:
    add_color("importantTooltipBg", get_element("toastBg"));
    // \/ [UNTESTED]:
    add_color("importantTooltipFg", get_element("toastFg"));
    // \/ [UNTESTED]:
    add_color("importantTooltipFgLink", colors["color2"].clone());

    add_comment("Bot");
    // \/ [UNTESTED]:
    add_color("botKbBg", colors["color0"].clone());
    // \/ [UNTESTED]:
    add_color("botKbDownBg", colors["color0"].borrow().lighter(40));

    add_comment("Overview");
    // \/ [UNTESTED]:
    add_color("overviewCheckBorder", colors["color2"].clone());

    add_comment("Sidebar");
    add_color("sideBarBg", colors["color0"].clone());
    add_color("sideBarBgActive", colors["color2"].clone());
    add_color("sideBarBgRipple", colors["color1"].clone());
    add_color("sideBarTextFg", colors["color1"].clone());
    add_color("sideBarTextFgActive", colors["color7"].clone());
    add_color("sideBarIconFg", colors["color7"].clone());
    add_color("sideBarIconFgActive", colors["color7"].borrow().lighter(40));
    add_color("sideBarBadgeBg", colors["color1"].clone());
    add_color("sideBarBadgeBgMuted", colors["color7"].borrow().darker(40));
    add_color("sideBarBadgeFg", colors["color7"].borrow().lighter(40));

    add_comment("DUNNO");
    // \/ [UNTESTED]:
    add_color("profileOtherAdminStarFg", colors["color7"].clone());

    return ret.borrow().clone();
}
