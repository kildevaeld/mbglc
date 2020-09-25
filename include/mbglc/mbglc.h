#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct mbgl_latlng
    {
        double lng;
        double lat;
    } mbgl_latlng_t;

    typedef enum mbgl_runloop_type
    {
        NEW_RUNLOOP,
        DEFAULT_RUNLOOP
    } mbgl_runloop_type;

    typedef struct mbgl_run_loop mbgl_run_loop_t;
    typedef struct mbgl_map mbgl_map_t;

    const char *mbgl_api_endpoint();

    mbgl_run_loop_t *mbgl_run_loop_create(mbgl_runloop_type);
    void mbgl_run_loop_close(mbgl_run_loop_t *);
    void mbgl_run_loop_free(mbgl_run_loop_t *);

    mbgl_map_t *mbgl_map_create(int w, int h, int pixel, const char *access, const char *cache_path, const char *assets_path);
    void mbgl_map_free(mbgl_map_t *);

    void mbgl_map_set_size(mbgl_map_t *, int width, int height);
    void mbgl_map_get_size(mbgl_map_t *, int *width, int *height);

    void mbgl_map_jump_to(mbgl_map_t *, mbgl_latlng_t center, double zoom);

    void mbgl_map_load_style_url(mbgl_map_t *, const char *url);

    char *mbgl_map_render(mbgl_map_t *, size_t *);


#ifdef __cplusplus
}
#endif