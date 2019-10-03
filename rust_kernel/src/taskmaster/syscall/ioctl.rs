use super::scheduler::SCHEDULER;
use super::Fd;
use super::SysResult;

use screen::{AdvancedGraphic, Drawer};
use terminal::SCREEN_MONAD;
use core::convert::TryFrom;
use libc_binding::{winsize, IoctlCmd};

pub fn sys_ioctl(_fildes: Fd, cmd: u32, arg: u32) -> SysResult<u32> {
    unpreemptible_context!({
        let cmd = IoctlCmd::try_from(cmd)?;
        let mut scheduler = SCHEDULER.lock();

        match cmd {
            IoctlCmd::TIOCGWINSZ => {
                let win = {
                    let v = scheduler
                        .current_thread_mut()
                        .unwrap_process_mut()
                        .get_virtual_allocator();

                    v.make_checked_ref_mut(arg as *mut winsize)
                }?;
                {
                    // we handle only one size of screen in all tty for the moment
                    let screen_monad = SCREEN_MONAD.lock();
                    let size = screen_monad.query_window_size();

                    win.ws_row = size.line as u16;
                    win.ws_col = size.column as u16;
                    if let Ok((height, width, bpp)) = screen_monad.query_graphic_infos() {
                        win.ws_xpixel = width as u16;
                        win.ws_ypixel = height as u16;
                        win.bpp = bpp as u16;
                    }
                }
                Ok(0)
            }
        }
    })
}
