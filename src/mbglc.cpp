#include <mbglc/mbglc.h>
#include <mbgl/gfx/backend.hpp>
#include <mbgl/gfx/headless_frontend.hpp>
#include <mbgl/map/map.hpp>
#include <mbgl/map/map_options.hpp>
#include <mbgl/style/style.hpp>
#include <mbgl/util/default_styles.hpp>
#include <mbgl/util/image.hpp>
#include <mbgl/layermanager/layer_manager.hpp>
#include <mbgl/util/run_loop.hpp>
#include <fstream>
#include <thread>
#include <chrono>

#include <iostream>

using namespace mbgl;

struct mbgl_map
{
    std::unique_ptr<HeadlessFrontend> frontend;
    std::unique_ptr<mbgl::Map> map;
    int render_retries;
};

struct mbgl_run_loop
{
    std::unique_ptr<mbgl::util::RunLoop> loop;
};

struct mbgl_buffer
{
    std::string buffer;
};

mbgl_run_loop_t *mbgl_run_loop_create(mbgl_runloop_type type)
{
    auto t = util::RunLoop::Type::Default;
    if (type == NEW_RUNLOOP)
    {
        t = util::RunLoop::Type::New;
    }

    auto loop = std::make_unique<util::RunLoop>(t);

    return new mbgl_run_loop{std::move(loop)};
}

const char *mbgl_api_endpoint()
{
    return util::API_BASE_URL;
}

void mbgl_run_loop_close(mbgl_run_loop_t *loop)
{
    loop->loop->stop();
}
void mbgl_run_loop_free(mbgl_run_loop_t *loop)
{
    if (!loop)
        return;
    delete loop;
}

mbgl_map_t *mbgl_map_create(int w, int h, int pixel, const char *access, const char *cache_path, const char *assets_path)
{

    auto frontend = std::make_unique<HeadlessFrontend>(Size{(uint32_t)w, (uint32_t)h}, pixel);

    ResourceOptions resource;

    if (access)
        resource.withAccessToken(access);
    if (cache_path)
        resource.withCachePath(cache_path);
    if (assets_path)
        resource.withAssetPath(assets_path);

    auto map = std::make_unique<Map>(*frontend, MapObserver::nullObserver(), MapOptions().withSize(frontend->getSize()).withMapMode(MapMode::Static), resource);
    return new mbgl_map_t{std::move(frontend), std::move(map), 20};
}

void mbgl_map_free(mbgl_map_t *map)
{
    if (!map)
        return;

    delete map;
}

void mbgl_map_load_style_url(mbgl_map_t *map, const char *url)
{
    map->map->getStyle().loadURL(url);
}

char *mbgl_map_render(mbgl_map_t *map, size_t *len)
{
    try
    {

        auto result = map->frontend->render(*map->map);
        if (result.stats.isZero() || !result.image.valid())
        {

            return NULL;
        }

        auto buf = encodePNG(result.image);
        *len = buf.size();

        char *ca = (char *)std::malloc(sizeof(char) * buf.size());
        std::copy(buf.begin(), buf.end() - 1, ca);

        return ca;
    }
    catch (std::exception &e)
    {
        return NULL;
    }
}

void mbgl_map_set_size(mbgl_map_t *map, int width, int height)
{
    map->frontend->setSize({(uint32_t)width, (uint32_t)height});
    map->map->setSize(map->frontend->getSize());
}
void mbgl_map_get_size(mbgl_map_t *map, int *width, int *height)
{
    auto size = map->frontend->getSize();
    *width = (int)size.width;
    *height = (int)size.height;
}

void mbgl_map_jump_to(mbgl_map_t *map, mbgl_latlng_t c, double zoom)
{
    LatLng center{c.lat, c.lng};
    map->map->jumpTo(CameraOptions().withCenter(center).withZoom(zoom));
}
