fn main() {
    let win_width = 100u16;
    let win_height = 100u16;
    let win_start_x = 0;
    let win_start_y = 0;

    let polyline: &[xcb::Point] = &[
        xcb::Point::new(100, 100),
        xcb::Point::new(0, -40),
        xcb::Point::new(10, 40),
        xcb::Point::new(0, -40),
    ];

    let polyarc: &[xcb::Arc] = &[xcb::Arc::new(
        win_start_x + 5,
        win_start_y + 5,
        win_width - 10,
        win_height - 10,
        0,
        360 << 6,
    )];

    let polyarc2: &[xcb::Arc] = &[xcb::Arc::new(
        win_start_x,
        win_start_y,
        win_width,
        win_height,
        0,
        360 << 6,
    )];

    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let foreground = conn.generate_id();

    xcb::create_gc(
        &conn,
        foreground,
        screen.root(),
        &[
            (xcb::GC_FOREGROUND, screen.black_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
            (xcb::GC_LINE_WIDTH, 1),
        ],
    );

    let win = conn.generate_id();
    xcb::create_window(
        &conn,
        xcb::COPY_FROM_PARENT as u8,
        win,
        screen.root(),
        win_start_x,
        win_start_y,
        win_width,
        win_height,
        0,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &[
            (xcb::CW_BACK_PIXEL, screen.white_pixel()),
            (
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS,
            ),
        ],
    );
    xcb::map_window(&conn, win);
    conn.flush();

    let reply_cookie = xcb::query_pointer(&conn, win);
    let reply = reply_cookie.get_reply();
    match reply {
        Ok(r) => {
            println!("X: {} Y: {}", r.root_x(), r.root_y());
        }
        Err(_) => {
            println!("Error!");
        }
    }

    loop {
        let event = conn.wait_for_event();
        match event {
            None => {
                break;
            }
            Some(event) => {
                let r = event.response_type() & !0x80;
                match r {
                    xcb::EXPOSE => {
                        xcb::poly_arc(&conn, win, foreground, &polyarc);
                        xcb::poly_arc(&conn, win, foreground, &polyarc2);
                        conn.flush();
                    }
                    _ => {}
                }
            }
        }
    }
    loop {}
}
