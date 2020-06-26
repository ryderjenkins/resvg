/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/**
 * @file ResvgQt.h
 *
 * Qt API for resvg-qt
 */

#ifndef RESVG_QT_H
#define RESVG_QT_H

#define RESVG_QT_MAJOR_VERSION 0
#define RESVG_QT_MINOR_VERSION 10
#define RESVG_QT_PATCH_VERSION 0
#define RESVG_QT_VERSION "0.10.0"

#include <QDebug>
#include <QFile>
#include <QGuiApplication>
#include <QPainter>
#include <QRectF>
#include <QScopedPointer>
#include <QScreen>
#include <QString>
#include <QTransform>

#include <resvg.h>

namespace ResvgPrivate {

extern "C" {
    void resvg_qt_render_to_canvas(const resvg_render_tree *tree,
                                   resvg_size size,
                                   void *painter);
    void resvg_qt_render_to_canvas_by_id(const resvg_render_tree *tree,
                                         resvg_size size,
                                         const char *id,
                                         void *painter);
}

class Data
{
public:
    Data()
    {
        init();
    }

    ~Data()
    {
        clear();
    }

    void reset()
    {
        clear();
        init();
    }

    resvg_render_tree *tree = nullptr;
    resvg_options *opt = nullptr;
    qreal scaleFactor = 1.0;
    QRectF viewBox;
    QString errMsg;

private:
    void init()
    {
        // Do not set the default font via QFont::family()
        // because it will return a dummy one on Windows.
        // See https://github.com/RazrFalcon/resvg/issues/159

        opt = resvg_options_create();

        auto languages = QLocale().bcp47Name().toUtf8();
        languages.append('\0');
        resvg_options_set_languages(opt, languages.constData());

        resvg_options_set_dpi(opt, 96 * scaleFactor);

        resvg_options_load_system_fonts(opt);
    }

    void clear()
    {
        // No need to deallocate opt.font_family, because it is a constant.

        if (tree) {
            resvg_tree_destroy(tree);
            tree = nullptr;
        }

        if (opt) {
            resvg_options_destroy(opt);
            tree = nullptr;
        }

        viewBox = QRectF();
        errMsg = QString();
    }
};

static QString errorToString(const int err)
{
    switch (err) {
        case RESVG_OK :
            return QString();
        case RESVG_ERROR_NOT_AN_UTF8_STR :
            return QLatin1String("The SVG content has not an UTF-8 encoding.");
        case RESVG_ERROR_FILE_OPEN_FAILED :
            return QLatin1String("Failed to read the file.");
        case RESVG_ERROR_INVALID_FILE_SUFFIX :
            return QLatin1String("Invalid file suffix.");
        case RESVG_ERROR_MALFORMED_GZIP :
            return QLatin1String("Not a GZip compressed data.");
        case RESVG_ERROR_INVALID_SIZE :
            return QLatin1String("SVG doesn't have a valid size.");
        case RESVG_ERROR_PARSING_FAILED :
            return QLatin1String("Failed to parse an SVG data.");
    }

    Q_UNREACHABLE();
}

} //ResvgPrivate

/**
 * @brief QSvgRenderer-like wrapper for resvg.
 */
class ResvgRenderer {
public:
    /**
     * @brief Constructs a new renderer.
     */
    ResvgRenderer();

    /**
     * @brief Constructs a new renderer and loads the contents of the SVG(Z) file.
     */
    ResvgRenderer(const QString &filePath);

    /**
     * @brief Constructs a new renderer and loads the SVG data.
     */
    ResvgRenderer(const QByteArray &data);

    /**
     * @brief Destructs the renderer.
     */
    ~ResvgRenderer();

    /**
     * @brief Loads the contents of the SVG(Z) file.
     */
    bool load(const QString &filePath);

    /**
     * @brief Loads the SVG data.
     */
    bool load(const QByteArray &data);

    /**
     * @brief Returns \b true if the file or data were loaded successful.
     */
    bool isValid() const;

    /**
     * @brief Returns an underling error when #isValid is \b false.
     */
    QString errorString() const;

    /**
     * @brief Checks that underling tree has any nodes.
     *
     * #ResvgRenderer and #ResvgRenderer constructors
     * will set an error only if a file does not exist or it has a non-UTF-8 encoding.
     * All other errors will result in an empty tree with a 100x100px size.
     *
     * @return Returns \b true if tree has any nodes.
     */
    bool isEmpty() const;

    /**
     * @brief Returns an SVG size.
     */
    QSize defaultSize() const;

    /**
     * @brief Returns an SVG size.
     */
    QSizeF defaultSizeF() const;

    /**
     * @brief Returns an SVG viewbox.
     */
    QRect viewBox() const;

    /**
     * @brief Returns an SVG viewbox.
     */
    QRectF viewBoxF() const;

    /**
     * @brief Returns bounding rectangle of the item with the given \b id.
     *        The transformation matrix of parent elements is not affecting
     *        the bounds of the element.
     */
    QRectF boundsOnElement(const QString &id) const;

    /**
     * @brief Returns bounding rectangle of a whole image.
     */
    QRectF boundingBox() const;

    /**
     * @brief Returns \b true if element with such an ID exists.
     */
    bool elementExists(const QString &id) const;

    /**
     * @brief Returns element's transform.
     */
    QTransform transformForElement(const QString &id) const;

    /**
     * @brief Sets the device pixel ratio for the image.
     */
    void setDevicePixelRatio(qreal scaleFactor);

    /**
     * @brief Renders the SVG data onto the canvas.
     *
     * \b Warning: the canvas must not have a transform.
     */
    void render(QPainter *p) const;

    /**
     * @brief Renders the SVG data to \b QImage with a specified \b size.
     *
     * If \b size is not set, the \b defaultSize() will be used.
     */
    QImage renderToImage(const QSize &size = QSize()) const;

    /**
     * @brief Initializes the library log.
     *
     * Use it if you want to see any warnings.
     *
     * Must be called only once.
     *
     * All warnings will be printed to the \b stderr.
     */
    static void initLog();

private:
    QScopedPointer<ResvgPrivate::Data> d;
};

// Implementation.

inline ResvgRenderer::ResvgRenderer()
    : d(new ResvgPrivate::Data())
{
}

inline ResvgRenderer::ResvgRenderer(const QString &filePath)
    : d(new ResvgPrivate::Data())
{
    load(filePath);
}

inline ResvgRenderer::ResvgRenderer(const QByteArray &data)
    : d(new ResvgPrivate::Data())
{
    load(data);
}

inline ResvgRenderer::~ResvgRenderer() {}

inline bool ResvgRenderer::load(const QString &filePath)
{
    // Check for Qt resource path.
    if (filePath.startsWith(QLatin1String(":/"))) {
        QFile file(filePath);
        if (file.open(QFile::ReadOnly)) {
            return load(file.readAll());
        } else {
            return false;
        }
    }

    d->reset();

    auto filePathC = filePath.toUtf8();
    filePathC.append('\0');
    resvg_options_set_file_path(d->opt, filePathC.constData());

    const auto err = resvg_parse_tree_from_file(filePathC.constData(), d->opt, &d->tree);
    if (err != RESVG_OK) {
        d->errMsg = ResvgPrivate::errorToString(err);
        return false;
    }

    const auto r = resvg_get_image_viewbox(d->tree);
    d->viewBox = QRectF(r.x, r.y, r.width, r.height);

    return true;
}

inline bool ResvgRenderer::load(const QByteArray &data)
{
    d->reset();

    const auto err = resvg_parse_tree_from_data(data.constData(), data.size(), d->opt, &d->tree);
    if (err != RESVG_OK) {
        d->errMsg = ResvgPrivate::errorToString(err);
        return false;
    }

    const auto r = resvg_get_image_viewbox(d->tree);
    d->viewBox = QRectF(r.x, r.y, r.width, r.height);

    return true;
}

inline bool ResvgRenderer::isValid() const
{
    return d->tree;
}

inline QString ResvgRenderer::errorString() const
{
    return d->errMsg;
}

inline bool ResvgRenderer::isEmpty() const
{
    if (d->tree)
        return !resvg_is_image_empty(d->tree);
    else
        return true;
}

inline QSize ResvgRenderer::defaultSize() const
{
    return defaultSizeF().toSize();
}

inline QSizeF ResvgRenderer::defaultSizeF() const
{
    if (d->tree)
        return d->viewBox.size();
    else
        return QSizeF();
}

inline QRect ResvgRenderer::viewBox() const
{
    return viewBoxF().toRect();
}

inline QRectF ResvgRenderer::viewBoxF() const
{
    if (d->tree)
        return d->viewBox;
    else
        return QRectF();
}

inline QRectF ResvgRenderer::boundsOnElement(const QString &id) const
{
    if (!d->tree)
        return QRectF();

    const auto utf8Str = id.toUtf8();
    const auto rawId = utf8Str.constData();
    resvg_rect bbox;
    if (resvg_get_node_bbox(d->tree, rawId, &bbox))
        return QRectF(bbox.x, bbox.y, bbox.height, bbox.width);

    return QRectF();
}

inline QRectF ResvgRenderer::boundingBox() const
{
    if (!d->tree)
        return QRectF();

    resvg_rect bbox;
    if (resvg_get_image_bbox(d->tree, &bbox))
        return QRectF(bbox.x, bbox.y, bbox.height, bbox.width);

    return QRectF();
}

inline bool ResvgRenderer::elementExists(const QString &id) const
{
    if (!d->tree)
        return false;

    const auto utf8Str = id.toUtf8();
    const auto rawId = utf8Str.constData();
    return resvg_node_exists(d->tree, rawId);
}

inline QTransform ResvgRenderer::transformForElement(const QString &id) const
{
    if (!d->tree)
        return QTransform();

    const auto utf8Str = id.toUtf8();
    const auto rawId = utf8Str.constData();
    resvg_transform ts;
    if (resvg_get_node_transform(d->tree, rawId, &ts))
        return QTransform(ts.a, ts.b, ts.c, ts.d, ts.e, ts.f);

    return QTransform();
}

inline void ResvgRenderer::setDevicePixelRatio(qreal scaleFactor)
{
    d->scaleFactor = scaleFactor;
}

// TODO: render node

inline void ResvgRenderer::render(QPainter *p) const
{
    if (!d->tree)
        return;

    p->save();
    p->setRenderHint(QPainter::Antialiasing);

    const auto r = p->viewport();
    resvg_size imgSize { (uint)r.width(), (uint)r.height() };
    ResvgPrivate::resvg_qt_render_to_canvas(d->tree, imgSize, p);

    p->restore();
}

inline QImage ResvgRenderer::renderToImage(const QSize &size) const
{
    const auto s = size.isValid() ? size : defaultSize();
    QImage img(s, QImage::Format_ARGB32_Premultiplied);
    img.fill(Qt::transparent);

    QPainter p(&img);
    render(&p);
    p.end();

    return img;
}

inline void ResvgRenderer::initLog()
{
    resvg_init_log();
}

#endif // RESVG_QT_H
