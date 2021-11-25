#include "cfl_flow.h"
#include "Fl_Flow.H"

#include <cfltk/cfl_lock.h>

#include <FL/Fl_Group.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Widget.H>
#include <FL/fl_draw.H>

WIDGET_CLASS(Fl_Flow)

WIDGET_DEFINE(Fl_Flow)

void Fl_Flow_rule(Fl_Flow* self, Fl_Widget* wid, const char* inst)
{
    LOCK(self->rule(wid, inst));
}

GROUP_DEFINE(Fl_Flow)