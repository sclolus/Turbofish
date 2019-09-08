#include <ltrace.h>
#include <errno.h>
#include <stdio.h>

#ifndef N_
# define N_(s) ((char *)s)
#endif

const char *const sys_errlist[] =
  {
    [0] = N_("Success"),
#ifdef EPERM
/*
 * Only the owner of the file (or other resource)
 * or processes with special privileges can perform the operation. */
    [EPERM] = N_("Operation not permitted"),
#endif
#ifdef ENOENT
/*
 * This is a ``file doesn't exist'' error
 * for ordinary files that are referenced in contexts where they are
 * expected to already exist. */
    [ENOENT] = N_("No such file or directory"),
#endif
#ifdef ESRCH
/*
 * No process matches the specified process ID. */
    [ESRCH] = N_("No such process"),
#endif
#ifdef EINTR
/*
 * An asynchronous signal occurred and prevented
 * completion of the call.  When this happens, you should try the call
 * again.
TRANS
 * You can choose to have functions resume after a signal that is handled,
 * rather than failing with @code{EINTR}; see @ref{Interrupted
 * Primitives}. */
    [EINTR] = N_("Interrupted system call"),
#endif
#ifdef EIO
/*
 * Usually used for physical read or write errors. */
    [EIO] = N_("Input/output error"),
#endif
#ifdef ENXIO
/*
 * The system tried to use the device
 * represented by a file you specified, and it couldn't find the device.
 * This can mean that the device file was installed incorrectly, or that
 * the physical device is missing or not correctly attached to the
 * computer. */
    [ENXIO] = N_("No such device or address"),
#endif
#ifdef E2BIG
/*
 * Used when the arguments passed to a new program
 * being executed with one of the @code{exec} functions (@pxref{Executing a
 * File}) occupy too much memory space.  This condition never arises on
 * @gnuhurdsystems{}. */
    [E2BIG] = N_("Argument list too long"),
#endif
#ifdef ENOEXEC
/*
 * Invalid executable file format.  This condition is detected by the
 * @code{exec} functions; see @ref{Executing a File}. */
    [ENOEXEC] = N_("Exec format error"),
#endif
#ifdef EBADF
/*
 * For example, I/O on a descriptor that has been
 * closed or reading from a descriptor open only for writing (or vice
 * versa). */
    [EBADF] = N_("Bad file descriptor"),
#endif
#ifdef ECHILD
/*
 * This error happens on operations that are
 * supposed to manipulate child processes, when there aren't any processes
 * to manipulate. */
    [ECHILD] = N_("No child processes"),
#endif
#ifdef EDEADLK
/*
 * Allocating a system resource would have resulted in a
 * deadlock situation.  The system does not guarantee that it will notice
 * all such situations.  This error means you got lucky and the system
 * noticed; it might just hang.  @xref{File Locks}, for an example. */
    [EDEADLK] = N_("Resource deadlock avoided"),
#endif
#ifdef ENOMEM
/*
 * The system cannot allocate more virtual memory
 * because its capacity is full. */
    [ENOMEM] = N_("Cannot allocate memory"),
#endif
#ifdef EACCES
/*
 * The file permissions do not allow the attempted operation. */
    [EACCES] = N_("Permission denied"),
#endif
#ifdef EFAULT
/*
 * An invalid pointer was detected.
 * On @gnuhurdsystems{}, this error never happens; you get a signal instead. */
    [EFAULT] = N_("Bad address"),
#endif
#ifdef ENOTBLK
/*
 * A file that isn't a block special file was given in a situation that
 * requires one.  For example, trying to mount an ordinary file as a file
 * system in Unix gives this error. */
    [ENOTBLK] = N_("Block device required"),
#endif
#ifdef EBUSY
/*
 * A system resource that can't be shared is already in use.
 * For example, if you try to delete a file that is the root of a currently
 * mounted filesystem, you get this error. */
    [EBUSY] = N_("Device or resource busy"),
#endif
#ifdef EEXIST
/*
 * An existing file was specified in a context where it only
 * makes sense to specify a new file. */
    [EEXIST] = N_("File exists"),
#endif
#ifdef EXDEV
/*
 * An attempt to make an improper link across file systems was detected.
 * This happens not only when you use @code{link} (@pxref{Hard Links}) but
 * also when you rename a file with @code{rename} (@pxref{Renaming Files}). */
    [EXDEV] = N_("Invalid cross-device link"),
#endif
#ifdef ENODEV
/*
 * The wrong type of device was given to a function that expects a
 * particular sort of device. */
    [ENODEV] = N_("No such device"),
#endif
#ifdef ENOTDIR
/*
 * A file that isn't a directory was specified when a directory is required. */
    [ENOTDIR] = N_("Not a directory"),
#endif
#ifdef EISDIR
/*
 * You cannot open a directory for writing,
 * or create or remove hard links to it. */
    [EISDIR] = N_("Is a directory"),
#endif
#ifdef EINVAL
/*
 * This is used to indicate various kinds of problems
 * with passing the wrong argument to a library function. */
    [EINVAL] = N_("Invalid argument"),
#endif
#ifdef EMFILE
/*
 * The current process has too many files open and can't open any more.
 * Duplicate descriptors do count toward this limit.
TRANS
 * In BSD and GNU, the number of open files is controlled by a resource
 * limit that can usually be increased.  If you get this error, you might
 * want to increase the @code{RLIMIT_NOFILE} limit or make it unlimited;
 * @pxref{Limits on Resources}. */
    [EMFILE] = N_("Too many open files"),
#endif
#ifdef ENFILE
/*
 * There are too many distinct file openings in the entire system.  Note
 * that any number of linked channels count as just one file opening; see
 * @ref{Linked Channels}.  This error never occurs on @gnuhurdsystems{}. */
    [ENFILE] = N_("Too many open files in system"),
#endif
#ifdef ENOTTY
/*
 * Inappropriate I/O control operation, such as trying to set terminal
 * modes on an ordinary file. */
    [ENOTTY] = N_("Inappropriate ioctl for device"),
#endif
#ifdef ETXTBSY
/*
 * An attempt to execute a file that is currently open for writing, or
 * write to a file that is currently being executed.  Often using a
 * debugger to run a program is considered having it open for writing and
 * will cause this error.  (The name stands for ``text file busy''.)  This
 * is not an error on @gnuhurdsystems{}; the text is copied as necessary. */
    [ETXTBSY] = N_("Text file busy"),
#endif
#ifdef EFBIG
/*
 * The size of a file would be larger than allowed by the system. */
    [EFBIG] = N_("File too large"),
#endif
#ifdef ENOSPC
/*
 * Write operation on a file failed because the
 * disk is full. */
    [ENOSPC] = N_("No space left on device"),
#endif
#ifdef ESPIPE
/*
 * Invalid seek operation (such as on a pipe). */
    [ESPIPE] = N_("Illegal seek"),
#endif
#ifdef EROFS
/*
 * An attempt was made to modify something on a read-only file system. */
    [EROFS] = N_("Read-only file system"),
#endif
#ifdef EMLINK
/*
 * The link count of a single file would become too large.
 * @code{rename} can cause this error if the file being renamed already has
 * as many links as it can take (@pxref{Renaming Files}). */
    [EMLINK] = N_("Too many links"),
#endif
#ifdef EPIPE
/*
 * There is no process reading from the other end of a pipe.
 * Every library function that returns this error code also generates a
 * @code{SIGPIPE} signal; this signal terminates the program if not handled
 * or blocked.  Thus, your program will never actually see @code{EPIPE}
 * unless it has handled or blocked @code{SIGPIPE}. */
    [EPIPE] = N_("Broken pipe"),
#endif
#ifdef EDOM
/*
 * Used by mathematical functions when an argument value does
 * not fall into the domain over which the function is defined. */
    [EDOM] = N_("Numerical argument out of domain"),
#endif
#ifdef ERANGE
/*
 * Used by mathematical functions when the result value is
 * not representable because of overflow or underflow. */
    [ERANGE] = N_("Numerical result out of range"),
#endif
#ifdef EAGAIN
/*
 * The call might work if you try again
 * later.  The macro @code{EWOULDBLOCK} is another name for @code{EAGAIN};
 * they are always the same in @theglibc{}.
 *
 * This error can happen in a few different situations:
 *
 * @itemize @bullet
 * @item
 * An operation that would block was attempted on an object that has
 * non-blocking mode selected.  Trying the same operation again will block
 * until some external condition makes it possible to read, write, or
 * connect (whatever the operation).  You can use @code{select} to find out
 * when the operation will be possible; @pxref{Waiting for I/O}.
 *
 * @strong{Portability Note:} In many older Unix systems, this condition
 * was indicated by @code{EWOULDBLOCK}, which was a distinct error code
 * different from @code{EAGAIN}.  To make your program portable, you should
 * check for both codes and treat them the same.
 *
 * @item
 * A temporary resource shortage made an operation impossible.  @code{fork}
 * can return this error.  It indicates that the shortage is expected to
 * pass, so your program can try the call again later and it may succeed.
 * It is probably a good idea to delay for a few seconds before trying it
 * again, to allow time for other processes to release scarce resources.
 * Such shortages are usually fairly serious and affect the whole system,
 * so usually an interactive program should report the error to the user
 * and return to its command loop.
 * @end itemize */
    [EAGAIN] = N_("Resource temporarily unavailable"),
#endif
#if defined (EWOULDBLOCK) && EWOULDBLOCK != EAGAIN
/*
 * In @theglibc{}, this is another name for @code{EAGAIN} (above).
 * The values are always the same, on every operating system.
TRANS
 * C libraries in many older Unix systems have @code{EWOULDBLOCK} as a
 * separate error code. */
    [EWOULDBLOCK] = N_("Operation would block"),
#endif
#ifdef EINPROGRESS
/*
 * An operation that cannot complete immediately was initiated on an object
 * that has non-blocking mode selected.  Some functions that must always
 * block (such as @code{connect}; @pxref{Connecting}) never return
 * @code{EAGAIN}.  Instead, they return @code{EINPROGRESS} to indicate that
 * the operation has begun and will take some time.  Attempts to manipulate
 * the object before the call completes return @code{EALREADY}.  You can
 * use the @code{select} function to find out when the pending operation
 * has completed; @pxref{Waiting for I/O}. */
    [EINPROGRESS] = N_("Operation now in progress"),
#endif
#ifdef EALREADY
/*
 * An operation is already in progress on an object that has non-blocking
 * mode selected. */
    [EALREADY] = N_("Operation already in progress"),
#endif
#ifdef ENOTSOCK
/*
 * A file that isn't a socket was specified when a socket is required. */
    [ENOTSOCK] = N_("Socket operation on non-socket"),
#endif
#ifdef EMSGSIZE
/*
 * The size of a message sent on a socket was larger than the supported
 * maximum size. */
    [EMSGSIZE] = N_("Message too long"),
#endif
#ifdef EPROTOTYPE
/*
 * The socket type does not support the requested communications protocol. */
    [EPROTOTYPE] = N_("Protocol wrong type for socket"),
#endif
#ifdef ENOPROTOOPT
/*
 * You specified a socket option that doesn't make sense for the
 * particular protocol being used by the socket.  @xref{Socket Options}. */
    [ENOPROTOOPT] = N_("Protocol not available"),
#endif
#ifdef EPROTONOSUPPORT
/*
 * The socket domain does not support the requested communications protocol
 * (perhaps because the requested protocol is completely invalid).
 * @xref{Creating a Socket}. */
    [EPROTONOSUPPORT] = N_("Protocol not supported"),
#endif
#ifdef ESOCKTNOSUPPORT
/*
 * The socket type is not supported. */
    [ESOCKTNOSUPPORT] = N_("Socket type not supported"),
#endif
#ifdef EOPNOTSUPP
/*
 * The operation you requested is not supported.  Some socket functions
 * don't make sense for all types of sockets, and others may not be
 * implemented for all communications protocols.  On @gnuhurdsystems{}, this
 * error can happen for many calls when the object does not support the
 * particular operation; it is a generic indication that the server knows
 * nothing to do for that call.
 */
    [EOPNOTSUPP] = N_("Operation not supported"),
#endif
#ifdef EPFNOSUPPORT
/*
 * The socket communications protocol family you requested is not supported.
 */
    [EPFNOSUPPORT] = N_("Protocol family not supported"),
#endif
#ifdef EAFNOSUPPORT
/*
 * The address family specified for a socket is not supported; it is
 * inconsistent with the protocol being used on the socket.  @xref{Sockets}.
 */
    [EAFNOSUPPORT] = N_("Address family not supported by protocol"),
#endif
#ifdef EADDRINUSE
/*
 * The requested socket address is already in use.  @xref{Socket Addresses}.
 */
    [EADDRINUSE] = N_("Address already in use"),
#endif
#ifdef EADDRNOTAVAIL
/*
 * The requested socket address is not available; for example, you tried
 * to give a socket a name that doesn't match the local host name.
 * @xref{Socket Addresses}.
 */
    [EADDRNOTAVAIL] = N_("Cannot assign requested address"),
#endif
#ifdef ENETDOWN
/*
 * A socket operation failed because the network was down.
 */
    [ENETDOWN] = N_("Network is down"),
#endif
#ifdef ENETUNREACH
/*
 * A socket operation failed because the subnet containing the remote host
 * was unreachable.
 */
    [ENETUNREACH] = N_("Network is unreachable"),
#endif
#ifdef ENETRESET
/*
 * A network connection was reset because the remote host crashed.
 */
    [ENETRESET] = N_("Network dropped connection on reset"),
#endif
#ifdef ECONNABORTED
/*
 * A network connection was aborted locally.
 */
    [ECONNABORTED] = N_("Software caused connection abort"),
#endif
#ifdef ECONNRESET
/*
 * A network connection was closed for reasons outside the control of the
 * local host, such as by the remote machine rebooting or an unrecoverable
 * protocol violation.
 */
    [ECONNRESET] = N_("Connection reset by peer"),
#endif
#ifdef ENOBUFS
/*
 * The kernel's buffers for I/O operations are all in use.  In GNU, this
 * error is always synonymous with @code{ENOMEM}; you may get one or the
 * other from network operations. */
    [ENOBUFS] = N_("No buffer space available"),
#endif
#ifdef EISCONN
/*
 * You tried to connect a socket that is already connected.
 * @xref{Connecting}. */
    [EISCONN] = N_("Transport endpoint is already connected"),
#endif
#ifdef ENOTCONN
/*
 * The socket is not connected to anything.  You get this error when you
 * try to transmit data over a socket, without first specifying a
 * destination for the data.  For a connectionless socket (for datagram
 * protocols, such as UDP), you get @code{EDESTADDRREQ} instead. */
    [ENOTCONN] = N_("Transport endpoint is not connected"),
#endif
#ifdef EDESTADDRREQ
/*
 * No default destination address was set for the socket.  You get this
 * error when you try to transmit data over a connectionless socket,
 * without first specifying a destination for the data with @code{connect}.
 */
    [EDESTADDRREQ] = N_("Destination address required"),
#endif
#ifdef ESHUTDOWN
/*
 * The socket has already been shut down.
 */
    [ESHUTDOWN] = N_("Cannot send after transport endpoint shutdown"),
#endif
#ifdef ETOOMANYREFS
/* */
    [ETOOMANYREFS] = N_("Too many references: cannot splice"),
#endif
#ifdef ETIMEDOUT
/*
 * A socket operation with a specified timeout received no response during
 * the timeout period.
 */
    [ETIMEDOUT] = N_("Connection timed out"),
#endif
#ifdef ECONNREFUSED
/*
 * A remote host refused to allow the network connection (typically because
 * it is not running the requested service).
 */
    [ECONNREFUSED] = N_("Connection refused"),
#endif
#ifdef ELOOP
/*
 * Too many levels of symbolic links were encountered in looking up a file name.
 * This often indicates a cycle of symbolic links.
 */
    [ELOOP] = N_("Too many levels of symbolic links"),
#endif
#ifdef ENAMETOOLONG
/*
 * Filename too long (longer than @code{PATH_MAX}; @pxref{Limits for
 * Files}) or host name too long (in @code{gethostname} or
 * @code{sethostname}; @pxref{Host Identification}). */
    [ENAMETOOLONG] = N_("File name too long"),
#endif
#ifdef EHOSTDOWN
/*
 * The remote host for a requested network connection is down. */
    [EHOSTDOWN] = N_("Host is down"),
#endif
#ifdef EHOSTUNREACH
/*
 * The remote host for a requested network connection is not reachable. */
    [EHOSTUNREACH] = N_("No route to host"),
#endif
#ifdef ENOTEMPTY
/*
 * Directory not empty, where an empty directory was expected.  Typically,
 * this error occurs when you are trying to delete a directory.
 */
    [ENOTEMPTY] = N_("Directory not empty"),
#endif
#ifdef EPROCLIM
/*
 * This means that the per-user limit on new process would be exceeded by
 * an attempted @code{fork}.  @xref{Limits on Resources}, for details on
 * the @code{RLIMIT_NPROC} limit.
 */
    [EPROCLIM] = N_("Too many processes"),
#endif
#ifdef EUSERS
/*
 * The file quota system is confused because there are too many users.
 * @c This can probably happen in a GNU system when using NFS.
 */
    [EUSERS] = N_("Too many users"),
#endif
#ifdef EDQUOT
/*
 * The user's disk quota was exceeded. */
    [EDQUOT] = N_("Disk quota exceeded"),
#endif
#ifdef ESTALE
/*
 * This indicates an internal confusion in the
 * file system which is due to file system rearrangements on the server host
 * for NFS file systems or corruption in other file systems.
 * Repairing this condition usually requires unmounting, possibly repairing
 * and remounting the file system. */
    [ESTALE] = N_("Stale file handle"),
#endif
#ifdef EREMOTE
/*
 * An attempt was made to NFS-mount a remote file system with a file name that
 * already specifies an NFS-mounted file.
 * (This is an error on some operating systems, but we expect it to work
 * properly on @gnuhurdsystems{}, making this error code impossible.) */
    [EREMOTE] = N_("Object is remote"),
#endif
#ifdef EBADRPC
/* */
    [EBADRPC] = N_("RPC struct is bad"),
#endif
#ifdef ERPCMISMATCH
/* */
    [ERPCMISMATCH] = N_("RPC version wrong"),
#endif
#ifdef EPROGUNAVAIL
/* */
    [EPROGUNAVAIL] = N_("RPC program not available"),
#endif
#ifdef EPROGMISMATCH
/* */
    [EPROGMISMATCH] = N_("RPC program version wrong"),
#endif
#ifdef EPROCUNAVAIL
/* */
    [EPROCUNAVAIL] = N_("RPC bad procedure for program"),
#endif
#ifdef ENOLCK
/*
 * This is used by the file locking facilities; see
 * @ref{File Locks}.  This error is never generated by @gnuhurdsystems{}, but
 * it can result from an operation to an NFS server running another
 * operating system.
 */
    [ENOLCK] = N_("No locks available"),
#endif
#ifdef EFTYPE
/*
 * The file was the wrong type for the
 * operation, or a data file had the wrong format.
TRANS
 * On some systems @code{chmod} returns this error if you try to set the
 * sticky bit on a non-directory file; @pxref{Setting Permissions}.
 */
    [EFTYPE] = N_("Inappropriate file type or format"),
#endif
#ifdef EAUTH
/* */
    [EAUTH] = N_("Authentication error"),
#endif
#ifdef ENEEDAUTH
/* */
    [ENEEDAUTH] = N_("Need authenticator"),
#endif
#ifdef ENOSYS
/*
 * This indicates that the function called is
 * not implemented at all, either in the C library itself or in the
 * operating system.  When you get this error, you can be sure that this
 * particular function will always fail with @code{ENOSYS} unless you
 * install a new version of the C library or the operating system.
 */
    [ENOSYS] = N_("Function not implemented"),
#endif
#if defined (ENOTSUP) && ENOTSUP != EOPNOTSUPP
/*
 * A function returns this error when certain parameter
 * values are valid, but the functionality they request is not available.
 * This can mean that the function does not implement a particular command
 * or option value or flag bit at all.  For functions that operate on some
 * object given in a parameter, such as a file descriptor or a port, it
 * might instead mean that only @emph{that specific object} (file
 * descriptor, port, etc.) is unable to support the other parameters given;
 * different file descriptors might support different ranges of parameter
 * values.
 * If the entire function is not available at all in the implementation,
 * it returns @code{ENOSYS} instead.
 */
    [ENOTSUP] = N_("Not supported"),
#endif
#ifdef EILSEQ
/*
 * While decoding a multibyte character the function came along an invalid
 * or an incomplete sequence of bytes or the given wide character is invalid. */
    [EILSEQ] = N_("Invalid or incomplete multibyte or wide character"),
#endif
#ifdef EBACKGROUND
/*
 * On @gnuhurdsystems{}, servers supporting the @code{term} protocol return
 * this error for certain operations when the caller is not in the
 * foreground process group of the terminal.  Users do not usually see this
 * error because functions such as @code{read} and @code{write} translate
 * it into a @code{SIGTTIN} or @code{SIGTTOU} signal.  @xref{Job Control},
 * for information on process groups and these signals. */
    [EBACKGROUND] = N_("Inappropriate operation for background process"),
#endif
#ifdef EDIED
/*
 * On @gnuhurdsystems{}, opening a file returns this error when the file is
 * translated by a program and the translator program dies while starting
 * up, before it has connected to the file. */
    [EDIED] = N_("Translator died"),
#endif
#ifdef ED
/*
 * The experienced user will know what is wrong.
 * @c This error code is a joke.  Its perror text is part of the joke.
 * @c Don't change it. */
    [ED] = N_("?"),
#endif
#ifdef EGREGIOUS
/*
 * You did @strong{what}? */
    [EGREGIOUS] = N_("You really blew it this time"),
#endif
#ifdef EIEIO
/*
 * Go home and have a glass of warm, dairy-fresh milk.
 * @c Okay.  Since you are dying to know, I'll tell you.
 * @c This is a joke, obviously.  There is a children's song which begins,
 * @c "Old McDonald had a farm, e-i-e-i-o."  Every time I see the (real)
 * @c errno macro EIO, I think about that song.  Probably most of my
 * @c compatriots who program on Unix do, too.  One of them must have stayed
 * @c up a little too late one night and decided to add it to Hurd or Glibc.
 * @c Whoever did it should be castigated, but it made me laugh.
 * @c  --jtobey@channel1.com
 * @c
 * @c "bought the farm" means "died".  -jtobey
 * @c
 * @c Translators, please do not translate this litteraly, translate it into
 * @c an idiomatic funny way of saying that the computer died. */
    [EIEIO] = N_("Computer bought the farm"),
#endif
#ifdef EGRATUITOUS
/*
 * This error code has no purpose. */
    [EGRATUITOUS] = N_("Gratuitous error"),
#endif
#ifdef EBADMSG
/* */
    [EBADMSG] = N_("Bad message"),
#endif
#ifdef EIDRM
/* */
    [EIDRM] = N_("Identifier removed"),
#endif
#ifdef EMULTIHOP
/* */
    [EMULTIHOP] = N_("Multihop attempted"),
#endif
#ifdef ENODATA
/* */
    [ENODATA] = N_("No data available"),
#endif
#ifdef ENOLINK
/* */
    [ENOLINK] = N_("Link has been severed"),
#endif
#ifdef ENOMSG
/* */
    [ENOMSG] = N_("No message of desired type"),
#endif
#ifdef ENOSR
/* */
    [ENOSR] = N_("Out of streams resources"),
#endif
#ifdef ENOSTR
/* */
    [ENOSTR] = N_("Device not a stream"),
#endif
#ifdef EOVERFLOW
/* */
    [EOVERFLOW] = N_("Value too large for defined data type"),
#endif
#ifdef EPROTO
/* */
    [EPROTO] = N_("Protocol error"),
#endif
#ifdef ETIME
/* */
    [ETIME] = N_("Timer expired"),
#endif
#ifdef ECANCELED
/*
 * An asynchronous operation was canceled before it
 * completed.  @xref{Asynchronous I/O}.  When you call @code{aio_cancel},
 * the normal result is for the operations affected to complete with this
 * error; @pxref{Cancel AIO Operations}. */
    [ECANCELED] = N_("Operation canceled"),
#endif
#ifdef EOWNERDEAD
/* */
    [EOWNERDEAD] = N_("Owner died"),
#endif
#ifdef ENOTRECOVERABLE
/* */
    [ENOTRECOVERABLE] = N_("State not recoverable"),
#endif
#ifdef ERESTART
/* */
    [ERESTART] = N_("Interrupted system call should be restarted"),
#endif
#ifdef ECHRNG
/* */
    [ECHRNG] = N_("Channel number out of range"),
#endif
#ifdef EL2NSYNC
/* */
    [EL2NSYNC] = N_("Level 2 not synchronized"),
#endif
#ifdef EL3HLT
/* */
    [EL3HLT] = N_("Level 3 halted"),
#endif
#ifdef EL3RST
/* */
    [EL3RST] = N_("Level 3 reset"),
#endif
#ifdef ELNRNG
/* */
    [ELNRNG] = N_("Link number out of range"),
#endif
#ifdef EUNATCH
/* */
    [EUNATCH] = N_("Protocol driver not attached"),
#endif
#ifdef ENOCSI
/* */
    [ENOCSI] = N_("No CSI structure available"),
#endif
#ifdef EL2HLT
/* */
    [EL2HLT] = N_("Level 2 halted"),
#endif
#ifdef EBADE
/* */
    [EBADE] = N_("Invalid exchange"),
#endif
#ifdef EBADR
/* */
    [EBADR] = N_("Invalid request descriptor"),
#endif
#ifdef EXFULL
/* */
    [EXFULL] = N_("Exchange full"),
#endif
#ifdef ENOANO
/* */
    [ENOANO] = N_("No anode"),
#endif
#ifdef EBADRQC
/* */
    [EBADRQC] = N_("Invalid request code"),
#endif
#ifdef EBADSLT
/* */
    [EBADSLT] = N_("Invalid slot"),
#endif
#if defined (EDEADLOCK) && EDEADLOCK != EDEADLK
/* */
    [EDEADLOCK] = N_("File locking deadlock error"),
#endif
#ifdef EBFONT
/* */
    [EBFONT] = N_("Bad font file format"),
#endif
#ifdef ENONET
/* */
    [ENONET] = N_("Machine is not on the network"),
#endif
#ifdef ENOPKG
/* */
    [ENOPKG] = N_("Package not installed"),
#endif
#ifdef EADV
/* */
    [EADV] = N_("Advertise error"),
#endif
#ifdef ESRMNT
/* */
    [ESRMNT] = N_("Srmount error"),
#endif
#ifdef ECOMM
/* */
    [ECOMM] = N_("Communication error on send"),
#endif
#ifdef EDOTDOT
/* */
    [EDOTDOT] = N_("RFS specific error"),
#endif
#ifdef ENOTUNIQ
/* */
    [ENOTUNIQ] = N_("Name not unique on network"),
#endif
#ifdef EBADFD
/* */
    [EBADFD] = N_("File descriptor in bad state"),
#endif
#ifdef EREMCHG
/* */
    [EREMCHG] = N_("Remote address changed"),
#endif
#ifdef ELIBACC
/* */
    [ELIBACC] = N_("Can not access a needed shared library"),
#endif
#ifdef ELIBBAD
/* */
    [ELIBBAD] = N_("Accessing a corrupted shared library"),
#endif
#ifdef ELIBSCN
/* */
    [ELIBSCN] = N_(".lib section in a.out corrupted"),
#endif
#ifdef ELIBMAX
/* */
    [ELIBMAX] = N_("Attempting to link in too many shared libraries"),
#endif
#ifdef ELIBEXEC
/* */
    [ELIBEXEC] = N_("Cannot exec a shared library directly"),
#endif
#ifdef ESTRPIPE
/* */
    [ESTRPIPE] = N_("Streams pipe error"),
#endif
#ifdef EUCLEAN
/* */
    [EUCLEAN] = N_("Structure needs cleaning"),
#endif
#ifdef ENOTNAM
/* */
    [ENOTNAM] = N_("Not a XENIX named type file"),
#endif
#ifdef ENAVAIL
/* */
    [ENAVAIL] = N_("No XENIX semaphores available"),
#endif
#ifdef EISNAM
/* */
    [EISNAM] = N_("Is a named type file"),
#endif
#ifdef EREMOTEIO
/* */
    [EREMOTEIO] = N_("Remote I/O error"),
#endif
#ifdef ENOMEDIUM
/* */
    [ENOMEDIUM] = N_("No medium found"),
#endif
#ifdef EMEDIUMTYPE
/* */
    [EMEDIUMTYPE] = N_("Wrong medium type"),
#endif
#ifdef ENOKEY
/* */
    [ENOKEY] = N_("Required key not available"),
#endif
#ifdef EKEYEXPIRED
/* */
    [EKEYEXPIRED] = N_("Key has expired"),
#endif
#ifdef EKEYREVOKED
/* */
    [EKEYREVOKED] = N_("Key has been revoked"),
#endif
#ifdef EKEYREJECTED
/* */
    [EKEYREJECTED] = N_("Key was rejected by service"),
#endif
#ifdef ERFKILL
/* */
    [ERFKILL] = N_("Operation not possible due to RF-kill"),
#endif
#ifdef EHWPOISON
/* */
    [EHWPOISON] = N_("Memory page has hardware error"),
#endif
  };

#define NERR \
  (sizeof sys_errlist/ sizeof sys_errlist[0])

const int sys_nerr = NERR;

void perror(const char *msg) {
	dprintf(2, "%s: errno = %d %s\n", msg, errno, sys_errlist[errno]);
}
