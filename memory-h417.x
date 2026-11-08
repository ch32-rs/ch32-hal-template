/*
 * Simplified CH32H417QEU6 (QingKe V3F / boot core) memory map for the
 * blink template.
 *
 * ch32-metapac's generated memory.x names the H417 RAM regions ITCM / DTCM /
 * SRAM_SHARED but never declares a region literally called `RAM`, which
 * qingke-rt's link.x needs. This file overrides that layout and maps the
 * zero-wait shared SRAM as `RAM`.
 *
 * For a dual-core / highcode layout (code running from shared SRAM, mailbox
 * reserved at the top), see examples/ch32h417/v3f in ch32-hal.
 */
MEMORY
{
    CODE   (rx)  : ORIGIN = 0x00000000, LENGTH =  960K /* USR boot alias */
    USR    (rwx) : ORIGIN = 0x08000000, LENGTH =  960K
    SYS    (rwx) : ORIGIN = 0x1FFF0000, LENGTH =   56K
    VND    (r)   : ORIGIN = 0x1FFFF700, LENGTH =   256
    OPT    (rw)  : ORIGIN = 0x1FFFF800, LENGTH =   256
    ITCM   (rwx) : ORIGIN = 0x200A0000, LENGTH =  128K
    DTCM   (rwx) : ORIGIN = 0x200C0000, LENGTH =  256K
    RAM    (rwx) : ORIGIN = 0x20100000, LENGTH =  512K /* shared SRAM */
}

REGION_ALIAS("FLASH", CODE);
REGION_ALIAS("REGION_TEXT", CODE);
REGION_ALIAS("REGION_RODATA", CODE);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);
