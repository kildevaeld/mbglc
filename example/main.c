#include <mbglc/mbglc.h>
#include <stdlib.h>
#include <stdio.h>

int main()
{

    char *token = getenv("MAPBOX_ACCESS_TOKEN");
    if (!token)
    {
        printf("MAPBOX_ACCESS_TOKEN not defined");
        exit(1);
    }

    mbgl_map_options_t opts = {
        .width = 512,
        .height = 512,
        .pixel_ratio = 1,
        .access_token = token,
        .cache_path = NULL,
        .assets_path = NULL,
        .base_url = NULL};

    mbgl_run_loop_t *loop = mbgl_run_loop_create(DEFAULT_RUNLOOP);

    mbgl_map_t *map = mbgl_map_create(1280, 1024, 1, token, NULL, NULL);

    mbgl_map_load_style_url(map, "mapbox://styles/mapbox/streets-v11");

    size_t len;
    char *image = mbgl_map_render_png(map, &len);

    if (image == NULL)
    {
        printf("could not render");
        return 1;
    }

    mbgl_map_free(map);
    mbgl_run_loop_free(loop);

    FILE *write_ptr;

    write_ptr = fopen("test.png", "wb"); // w for write, b for binary

    fwrite(image, len, 1, write_ptr);

    free(image);
    fflush(write_ptr);
    fclose(write_ptr);

    return 0;
}