#include <global>

#ifdef VERTEX
attribute vec2 i_start_pos;
attribute float i_start_time;
attribute float i_finish_time;
attribute vec2 i_speed;
vec2 unit_pos() {
    float passed_time = min(u_time, i_finish_time) - i_start_time;
    return i_start_pos + i_speed * passed_time;
}
#endif