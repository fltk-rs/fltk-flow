#ifndef __CFL_FLOW_H__
#define __CFL_FLOW_H__

#include <cfltk/cfl_macros.h>

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Flow)

void Fl_Flow_rule(Fl_Flow *self, Fl_Widget *wid, const char *inst);

GROUP_DECLARE(Fl_Flow)

#ifdef __cplusplus
}
#endif
#endif