
#include "../../kernel/lib/types.h"

struct VESA_INFO
{
    char    VESASignature[4];     // times 4 db 0; 'VESA', signature de 4 octets
    u16     VESAVersion;          // dw 0; numéro de version de VBE
    char*   OEMStringPtr;         // dd 0; Pointeur vers le nom de l'OEM
    u32     Capabilities;         // dd 0; Possibilités de la carte graphique
    char*   VideoModePtr;         // dd 0; Pointeur vers les modes accessibles
    u16     TotalMemory;          // dw 0; Nombre de blocs mémoire de 64ko

    char    reserved[236];        // reserved: ===> times 236 db 0; Complément à 256 octets, taille du blocs

/*** Informations d'un mode vidéo ***/
    u16     ModeAttributes;       // dw 0; Attributs du mode
    u8      A_Attribute;          // db 0; Attibuts de la fenêtre A
    u8      WinBAttributes;       // db 0; Attibuts de la fenêtre Bd
    u16     WinGranularity;       // dw 0; Granularité de la fenêtre en ko
    u16     WinSize;              // dw 0; Taille de la fenêtre en ko
    u16     WinASegment;          // dw 0; Segment de la fenêtre A
    u16     WinBSegment;          // dw 0; Segment de la fenêtre B
    void*   WinFuncPtr;           // dd 0; Pointeur vers la fonction "de fenêtrage"
    u16     BytesPerScanLine;     // dw 0; Octets par "scanline"
    u16     XResolution;          // dw 0; Résolution horizontale
    u16     YResolution;          // dw 0; Résolution vertical
    u8      XCharSize;            // db 0; Largeur d'un caractère
    u8      YCharSize;            // db 0; Hauteur d'un caractère
    u8      NumberOfPlanes;       // db 0; Nombre de plans mémoire
    u8      BitsPerPixel;         // db 0; Bits par pixel
    u8      NumberOfBanks;        // db 0; Nombre de banques de style CGA
    u8      MemoryModel;          // db 0; Type de modèle mémoire
    u8      BankSize;             // db 0; Taille des banques de style CGA
    u8      NumberOfImagePages;   // db 0; Nombre de pages image
    u8      res1;                 // db 0; Reservé
    u8      RedMaskSize;          // db 0; Taille du masque rouge en couleur directe
    u8      RedFieldPosition;     // db 0; Position du bit faible du masque rouge
    u8      GreenMaskSize;        // db 0; Taille du masque vert en couleur directe
    u8      GreenFieldPosition;   // db 0; Position du bit faible du masque vert
    u8      BlueMaskSize;         // db 0; Taille du masque bleu en couleur directe
    u8      BlueFieldPosition;    // db 0; Position du bit faible du masque bleu
    u8      RsvdMaskSize;         // db 0; Taille du masque réservé en couleur directe
    u8      RsvdFieldPosition;    // db 0;  Position du bit faible du masque réservé
    u8      DirectColorModeInfo;  // db 0; Attributs du mode de couleur directe
    u32     FlatMemory;           /* dd 0; *//*** ADDRESSE DU FRAME LINEAR BUFFER ***/

    char    reserved_2[208];      // reserved 2 ====> times 208 db 0; Complément à 256 octets, taille du bloc

}       __attribute__ ((packed));

struct VESA_INFO screen;


void copy_Vesa_Info(int,struct VESA_INFO*);

//%define graphic_text_line_for_32b_kernel    (8192+512)                                      ; Utilisé pour le passage de la ligne courante de texte au noyau 32bits.
//%define graphic_text_colomn_for_32b_kernel  (graphic_text_line_for_32b_kernel + 1)

/***                                                                        ;Informations du pilote VESA
VESASignature:      times 4 db 0; 'VESA', signature de 4 octets
VESAVersion:        dw 0; numéro de version de VBE
OEMStringPtr:       dd 0; Pointeur vers le nom de l'OEM
Capabilities:       dd 0; Possibilités de la carte graphique
VideoModePtr:       dd 0; Pointeur vers les modes accessibles
TotalMemory:        dw 0; Nombre de blocs mémoire de 64ko
reserved:           times 236 db 0; Complément à 256 octets, taille du blocs
;Informations d'un mode vidéo
ModeAttributes:     dw 0; Attributs du mode
WinAAttributes:     db 0; Attibuts de la fenêtre A
WinBAttributes:     db 0; Attibuts de la fenêtre Bd
WinGranularity:     dw 0; Granularité de la fenêtre en ko
WinSize:            dw 0; Taille de la fenêtre en ko
WinASegment:        dw 0; Segment de la fenêtre A
WinBSegment:        dw 0; Segment de la fenêtre B
WinFuncPtr:         dd 0; Pointeur vers la fonction "de fenêtrage"
BytesPerScanLine:   dw 0; Octets par "scanline"
XResolution:        dw 0; Résolution horizontale
YResolution:        dw 0; Résolution vertical
XCharSize:          db 0; Largeur d'un caractère
YCharSize:          db 0; Hauteur d'un caractère
NumberOfPlanes:     db 0; Nombre de plans mémoire
BitsPerPixel:       db 0; Bits par pixel
NumberOfBanks:      db 0; Nombre de banques de style CGA
MemoryModel:        db 0; Type de modèle mémoire
BankSize:           db 0; Taille des banques de style CGA
NumberOfImagePages: db 0; Nombre de pages image
res1:               db 0; Reservé
RedMaskSize:        db 0; Taille du masque rouge en couleur directe
RedFieldPosition:   db 0; Position du bit faible du masque rouge
GreenMaskSize:      db 0; Taille du masque vert en couleur directe
GreenFieldPosition: db 0; Position du bit faible du masque vert
BlueMaskSize:       db 0; Taille du masque bleu en couleur directe
BlueFieldPosition:  db 0; Position du bit faible du masque bleu
RsvdMaskSize:       db 0; Taille du masque réservé en couleur directe
RsvdFieldPosition:  db 0;  Position du bit faible du masque réservé
DirectColorModeInfo:db 0; Attributs du mode de couleur directe
FlatMemory:         dd 0
res2:               times 208 db 0; Complément à 256 octets, taille du bloc */
