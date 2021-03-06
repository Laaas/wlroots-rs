//! Manager for wl_shell clients.

use libc;
use wayland_sys::server::signal::wl_signal_add;
use wlroots_sys::wlr_wl_shell_surface;

use super::wl_shell_handler::WlShell;
use {Surface, WlShellHandler, WlShellSurface};
use compositor::{Compositor, COMPOSITOR_PTR};

/// Handles making new Wayland shells as reported by clients.
pub trait WlShellManagerHandler {
    /// Callback that is triggerde when a new wayland shell surface appears.
    fn new_surface(&mut self,
                   &mut Compositor,
                   &mut WlShellSurface,
                   &mut Surface)
                   -> Option<Box<WlShellHandler>>;
}

wayland_listener!(WlShellManager, Box<WlShellManagerHandler>, [
    add_listener => add_notify: |this: &mut WlShellManager, data: *mut libc::c_void,| unsafe {
        let manager = &mut this.data;
        let data = data as *mut wlr_wl_shell_surface;
        wlr_log!(L_DEBUG, "New wl_shell_surface request {:p}", data);
        let compositor = &mut *COMPOSITOR_PTR;
        let mut surface = Surface::new((*data).surface);
        let mut shell_surface = WlShellSurface::new(data);
        surface.set_lock(true);
        shell_surface.set_lock(true);
        let new_surface_res = manager.new_surface(compositor, &mut shell_surface, &mut surface);
        surface.set_lock(false);
        shell_surface.set_lock(false);
        if let Some(shell_surface_handler) = new_surface_res {
            let mut shell_surface = WlShell::new((shell_surface, surface, shell_surface_handler));
            // Add the destroy event to this handler.
            wl_signal_add(&mut (*data).events.destroy as *mut _ as _,
                          shell_surface.destroy_listener() as _);

            // Add the ping timeout event to this handler.
            wl_signal_add(&mut (*data).events.ping_timeout as *mut _ as _,
                          shell_surface.ping_timeout_listener() as _);

            // Add the move request event to this handler.
            wl_signal_add(&mut (*data).events.request_move as *mut _ as _,
                          shell_surface.request_move_listener() as _);

            // Add the resize request event to this handler.
            wl_signal_add(&mut (*data).events.request_resize as *mut _ as _,
                          shell_surface.request_resize_listener() as _);

            // Add the fullscreen request event to this handler.
            wl_signal_add(&mut (*data).events.request_fullscreen as *mut _ as _,
                          shell_surface.request_fullscreen_listener() as _);

            // Add the maximize request event to this handler.
            wl_signal_add(&mut (*data).events.request_maximize as *mut _ as _,
                          shell_surface.request_maximize_listener() as _);

            // Add the set state request event to this handler.
            wl_signal_add(&mut (*data).events.set_state as *mut _ as _,
                          shell_surface.set_state_listener() as _);

            // Add the set title request event to this handler.
            wl_signal_add(&mut (*data).events.set_title as *mut _ as _,
                          shell_surface.set_title_listener() as _);

            // Add the set class request event to this handler.
            wl_signal_add(&mut (*data).events.set_class as *mut _ as _,
                          shell_surface.set_class_listener() as _);

            // TODO FIXME This is wrong, should be the same as input/output!
            // NOTE This is cleaned up in the wl_shell_handler::destroy signal.
            ::std::mem::forget(shell_surface);
        }
    };
]);
