#ifdef __APPLE__
    // Apple's included ncurses library is actually ncursesw.
    #define _XOPEN_SOURCE_EXTENDED
    #include <ncurses.h>
#else
    #include <ncursesw.h>
#endif

// Workaround for rust-bindgen#753
const attr_t _A_BOLD = A_BOLD;
#undef A_BOLD
const attr_t A_BOLD = _A_BOLD;

const attr_t _A_BLINK = A_BLINK;
#undef A_BLINK
const attr_t A_BLINK = _A_BLINK;
