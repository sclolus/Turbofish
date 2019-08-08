#ifndef __TERMIOS_H__
# define __TERMIOS_H__

//    The <termios.h> header shall contain the definitions used by the terminal I/O interfaces (see General Terminal Interface for the structures and names defined).
//    The termios Structure
//
//    The <termios.h> header shall define the following data types through typedef:

typedef int    cc_t;
//        Used for terminal special characters.
typedef int    speed_t;
//        Used for terminal baud rates.
typedef unsigned int    tcflag_t;
//        Used for terminal modes.

//    The above types shall be all unsigned integer types.

//    The implementation shall support one or more programming environments in which the widths of cc_t, speed_t, and tcflag_t are no greater than the width of type long. The names of these programming environments can be obtained using the confstr() function or the getconf utility.

//    The <termios.h> header shall define the termios structure, which shall include at least the following members:

//    The <termios.h> header shall define the following symbolic constant:

#define    NCCS 42

struct termios {
    tcflag_t  c_iflag   ; //  Input modes. 
    tcflag_t  c_oflag   ; //  Output modes. 
    tcflag_t  c_cflag   ; //  Control modes. 
    tcflag_t  c_lflag   ; //  Local modes. 
    cc_t      c_cc[NCCS]; //  Control characters. 
};

//        Size of the array c_cc for control characters.

//    The <termios.h> header shall define the following symbolic constants for use as subscripts for the array c_cc:

//    Subscript Usage
//The subscript values shall be suitable for use in #if preprocessing directives and shall be distinct, except that the VMIN and VTIME subscripts may have the same values as the VEOF and VEOL subscripts, respectively.
//    Input Modes

//    The <termios.h> header shall define the following symbolic constants for use as flags in the c_iflag field. The c_iflag field describes the basic terminal input control.

//    BRKINT
//        Signal interrupt on break.
//    ICRNL
//        Map CR to NL on input.
//    IGNBRK
//        Ignore break condition.
//    IGNCR
//        Ignore CR.
//    IGNPAR
//        Ignore characters with parity errors.
//    INLCR
//        Map NL to CR on input.
//    INPCK
//        Enable input parity check.
//    ISTRIP
//        Strip character.
//    IXANY
//        Enable any character to restart output.
//    IXOFF
//        Enable start/stop input control.
//    IXON
//        Enable start/stop output control.
//    PARMRK
//        Mark parity errors.
//
//    Output Modes
//
//    The <termios.h> header shall define the following symbolic constants for use as flags in the c_oflag field. The c_oflag field specifies the system treatment of output.
//
//    OPOST
//        Post-process output.
//    ONLCR
//        [XSI] [Option Start] Map NL to CR-NL on output. [Option End]
//    OCRNL
//        [XSI] [Option Start] Map CR to NL on output. [Option End]
//    ONOCR
//        [XSI] [Option Start] No CR output at column 0. [Option End]
//    ONLRET
//        [XSI] [Option Start] NL performs CR function. [Option End]
//    OFDEL
//        [XSI] [Option Start] Fill is DEL. [Option End]
//    OFILL
//        [XSI] [Option Start] Use fill characters for delay. [Option End]
//    NLDLY
//        [XSI] [Option Start] Select newline delays:
//
//        NL0
//            Newline type 0.
//        NL1
//            Newline type 1.
//
//        [Option End]
//    CRDLY
//        [XSI] [Option Start] Select carriage-return delays:
//
//        CR0
//            Carriage-return delay type 0.
//        CR1
//            Carriage-return delay type 1.
//        CR2
//            Carriage-return delay type 2.
//        CR3
//            Carriage-return delay type 3.
//
//        [Option End]
//    TABDLY
//        [XSI] [Option Start] Select horizontal-tab delays:
//
//        TAB0
//            Horizontal-tab delay type 0.
//        TAB1
//            Horizontal-tab delay type 1.
//        TAB2
//            Horizontal-tab delay type 2.
//        TAB3
//            Expand tabs to spaces.
//
//        [Option End]
//    BSDLY
//        [XSI] [Option Start] Select backspace delays:
//
//        BS0
//            Backspace-delay type 0.
//        BS1
//            Backspace-delay type 1.
//
//        [Option End]
//    VTDLY
//        [XSI] [Option Start] Select vertical-tab delays:
//
//        VT0
//            Vertical-tab delay type 0.
//        VT1
//            Vertical-tab delay type 1.
//
//        [Option End]
//    FFDLY
//        [XSI] [Option Start] Select form-feed delays:
//
//        FF0
//            Form-feed delay type 0.
//        FF1
//            Form-feed delay type 1.
//
//        [Option End]
//
//    Baud Rate Selection
//
//    The <termios.h> header shall define the following symbolic constants for use as values of objects of type speed_t.
//
//    The input and output baud rates are stored in the termios structure. These are the valid values for objects of type speed_t. Not all baud rates need be supported by the underlying hardware.
//
//    B0
//        Hang up
//    B50
//        50 baud
//    B75
//        75 baud
//    B110
//        110 baud
//    B134
//        134.5 baud
//    B150
//        150 baud
//    B200
//        200 baud
//    B300
//        300 baud
//    B600
//        600 baud
//    B1200
//        1200 baud
//    B1800
//        1800 baud
//    B2400
//        2400 baud
//    B4800
//        4800 baud
//    B9600
//        9600 baud
//    B19200
//        19200 baud
//    B38400
//        38400 baud
//
//    Control Modes
//
//    The <termios.h> header shall define the following symbolic constants for use as flags in the c_cflag field. The c_cflag field describes the hardware control of the terminal; not all values specified are required to be supported by the underlying hardware.
//
//    CSIZE
//        Character size:
//
//        CS5
//            5 bits
//        CS6
//            6 bits
//        CS7
//            7 bits
//        CS8
//            8 bits
//
//    CSTOPB
//        Send two stop bits, else one.
//    CREAD
//        Enable receiver.
//    PARENB
//        Parity enable.
//    PARODD
//        Odd parity, else even.
//    HUPCL
//        Hang up on last close.
//    CLOCAL
//        Ignore modem status lines.
//
//    The implementation shall support the functionality associated with the symbols CS7, CS8, CSTOPB, PARODD, and PARENB.
//    Local Modes
//
//    The <termios.h> header shall define the following symbolic constants for use as flags in the c_lflag field. The c_lflag field of the argument structure is used to control various terminal functions.
//
#define    ECHO 1
//        Enable echo.
//    ECHOE
//        Echo erase character as error-correcting backspace.
//    ECHOK
//        Echo KILL.
//    ECHONL
//        Echo NL.
#define   ICANON (1 << 1)
//        Canonical input (erase and kill processing).
//    IEXTEN
//        Enable extended input character processing.
#define    ISIG (1 << 2)
//        Enable signals.
//    NOFLSH
//        Disable flush after interrupt or quit.
//    TOSTOP
//        Send SIGTTOU for background output.
//
//    Attribute Selection
//
//    The <termios.h> header shall define the following symbolic constants for use with tcsetattr():
//
#define       TCSANOW 0
//        Change attributes immediately.
#define       TCSADRAIN 1
//        Change attributes when output has drained.
#define    TCSAFLUSH 2
//        Change attributes when output has drained; also flush pending input.
//
//    Line Control
//
//    The <termios.h> header shall define the following symbolic constants for use with tcflush():
//
//    TCIFLUSH
//        Flush pending input.
//    TCIOFLUSH
//        Flush both pending input and untransmitted output.
//    TCOFLUSH
//        Flush untransmitted output.
//
//    The <termios.h> header shall define the following symbolic constants for use with tcflow():
//
//    TCIOFF
//        Transmit a STOP character, intended to suspend input data.
//    TCION
//        Transmit a START character, intended to restart input data.
//    TCOOFF
//        Suspend output.
//    TCOON
//        Restart output.
//
//    The <termios.h> header shall define the pid_t type as described in <sys/types.h>.

#include <sys/types.h>
//
//    The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.
//
//    speed_t cfgetispeed(const struct termios *);

speed_t cfgetospeed(const struct termios *);
int     cfsetispeed(struct termios *, speed_t);
int     cfsetospeed(struct termios *, speed_t);
int     tcdrain(int);
int     tcflow(int, int);
int     tcflush(int, int);
int     tcgetattr(int, struct termios *);
pid_t   tcgetsid(int);
int     tcsendbreak(int, int);
int     tcsetattr(int, int, const struct termios *);

#endif
