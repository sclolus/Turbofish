#!/usr/bin/perl -w

#
# bdf2psf.pl
#   Generates a console .psf font from
#   an iso10646-1-encoded .bdf console-sized font, using .sfm as
#   an encoding table.
#
#   This script doesn't add the table itself to the font -- use "psfaddtable"
#   to do this:
#       bdf2psf.pl -s map.sfm source.bdf | psfaddtable - map.sfm -o font.psf
#
# Usage:
#   bdf2psf.pl -s map.sfm [-o font.psf] [sourcefont.bdf]
#
# Notes:
#   Serge Winitzki's bdf2fon.pl was used as a starting point for
#   .bdf parsing code.
#   Doesn't perform strict .bdf syntax/consistency checks!
#
# Author:
#   Dmitry Bolkhovityanov <bolkhov@inp.nsk.su>
#
# License:
#   BSD style
#

use strict;

my $sourcefont = undef;
my $destfont   = undef;
my $fontmap    = undef;

my @bitmaps = ();  # Ready-to-use console bitmaps
my @binfont;       # The font
my $is512   = 0;   # 0 -- 256-char font, 1 -- 512-char


my ($FBBx, $FBBy, $Xoff, $Yoff);

my $dummy;

sub ByteOf($)
{
    return pack("c", $_[0]);
}

sub ParseCommandline()
{
  my $x;
  my $a;
  my $o;

    if ($#ARGV < 0)
    {
        print "Usage:\n";
        print "  $0 -s map.sfm [-o font.psf] [sourcefont.bdf]\n";
        exit (1);
    }

    $x = 0;
    while ($x <= $#ARGV)
    {
        $a = $ARGV[$x++];

        if ($a =~ /^-/)
        {
            if ($x <= $#ARGV)
            {
                $o = $ARGV[$x++];
            }
            else
            {
                die "$0: option ('$a') should have a parameter\n";
            }

            if    ($a eq "-s")
            {
                die "$0: duplicate '$a' option\n" if defined($fontmap);
                $fontmap = $o;
            }
            elsif ($a eq "-o")
            {
                die "$0: duplicate '$a' option\n" if defined($destfont);
                $destfont = $o;
            }
            else
            {
                die "$0: unknown option '$a'\n";
            }
        }
        else
        {
            die "$0: duplicate source font specification" if defined($sourcefont);
            $sourcefont = $a;
        }
    }

    # Set default values
    $sourcefont = "-" unless defined($sourcefont);
    $fontmap    = "-" unless defined($fontmap);
    $destfont   = "-" unless defined($destfont);
    
    # Do sanity checking
    die "$0: can't read both source-font and font-map from stdin\n"
        if $sourcefont eq "-"  &&  $fontmap eq "-";
}

sub ReadBDF()
{
  my $encoding;
  my ($BBw, $BBh, $BBxoff0x, $BByoff0y);
  my $empties;
  my $y;
    
    open (BDF, "<$sourcefont")  ||
        die "$0: unable to open source font \"$sourcefont\": $!\n";

    READHEADER: while (<BDF>)
    {
        /^FONTBOUNDINGBOX\s/ &&
            do {
                ($dummy, $FBBx, $FBBy, $Xoff, $Yoff) = (split);

                ($FBBx == 8  ||  $FBBx == 9) ||
                    die "$0: font width (FBBx) is $FBBx, not 8 nor 9; unable to use it.\n";

                $FBBx = 8;
            };
        /^CHARS\s/ && last READHEADER;
    }

    READCHARS: while (<BDF>)
    {
        /^ENCODING\s+([0-9]+)\s*$/ &&
            do {
                $encoding = $1;
                $bitmaps[$encoding] = "";  # Initialize glyph bitmap
            };
        /^BBX\s/ &&
            do {
                ($dummy, $BBw, $BBh, $BBxoff0x, $BByoff0y) = (split);
            };
        /^BITMAP/ &&
            do {
                # First, add empty lines at top
                $empties = $FBBy+$Yoff - $BByoff0y - $BBh;
                $bitmaps[$encoding] .= ByteOf(0) x $empties  if $empties > 0;
                # Second, scan the hex bitmap
                for ($y = 0; $y < $BBh; $y++)
                {
                    $_ = <BDF>;
                    #print STDERR "_=$_, s=", substr($_, 0, 2), "\n";
                    $bitmaps[$encoding] .=
                        ByteOf((hex(substr($_, 0, 2))) >> $BBxoff0x*1);
                }
                # Third, add empty lines at bottom
                $empties = $FBBy - $BBh - $empties;
                $bitmaps[$encoding] .= ByteOf(0) x $empties  if $empties > 0;
            };
        /^ENDFONT/ && last READCHARS;
    }
    
    close(BDF);
}


sub ReadSFM()
{
  my ($slot, $uni);

    # Initialize font
    @binfont = (ByteOf(0) x $FBBy) x 512;
        
    open (SFM, "<$fontmap")  || 
        die "$0: unable to open fontmap \"$fontmap\": $!\n";

    READSFM: while (<SFM>)
    {
        /^\s*\#/  &&  next READSFM;
        /^\s*$/   &&  next READSFM;

        # $dummy-related stuff is for UniCyrX-like sfms, where first
        # u+codes for 0x00-0x1f slots are in the u+0000-u+001f range
        ($slot, $uni, $dummy) = (split);
        if ((oct($slot) <= 0x1f  ||  oct($slot) == 0x7f)         &&
            $uni =~ /^u\+00[0-9a-f][0-9a-f]$/i                   &&
            lc(substr($slot, -2)) eq lc(substr($uni, -2))        &&
            $dummy =~ /^u\+[0-9a-f][0-9a-f][0-9a-f][0-9a-f]/i)
        {
            $uni = $dummy;
        }

        # Check $uni for "u+xxxx" compliance
        if ($uni !~ /^u\+[0-9a-f][0-9a-f][0-9a-f][0-9a-f]$/i)
        {
            print STDERR "$0: map \"$fontmap\".$.: unrecognized uni=\"$uni\"\n";
            next READSFM;
        }

        # Convert to numeric values
        $slot = oct($slot);
        $uni  = hex(substr($uni, 2));

        # Check the slot
        printf STDERR "$0: map \"$fontmap\".$.: out-of-range slot 0x%x", $slot if $slot < 0  ||  $slot > 0x1FF;
        $is512 = 1 if $slot > 0xFF;
        
        # Copy requested glyph
        if (defined($bitmaps[$uni]))
        {
            $binfont[$slot] = $bitmaps[$uni];
        }
        else
        {
            printf STDERR "$0: missing glyph U+%04x for slot 0x%02x\n", $uni, $slot;
        }
    }
    
    close(SFM);
}


sub WritePSF()
{
    open (PSF, ">$destfont")  ||
        die "$0: unable to open \"$destfont\" for output: $!\n";

    print PSF ByteOf(0x36), ByteOf(0x04),
              ByteOf($is512? 0x01:0x00), ByteOf($FBBy),
              @binfont[0..255+256*$is512];
    
    close(PSF);
}

######################################################################
#                                                                    #
#  Main                                                              #
#                                                                    #
######################################################################

ParseCommandline();
ReadBDF();
ReadSFM();
WritePSF();
