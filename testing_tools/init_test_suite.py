#!/usr/bin/env python3

import urllib.request
import os
import tarfile
import shutil


out_of_scope_tests = [
    'animate-dom-01-f.svg',
    'animate-dom-02-f.svg',
    'animate-elem-02-t.svg',
    'animate-elem-03-t.svg',
    'animate-elem-04-t.svg',
    'animate-elem-05-t.svg',
    'animate-elem-06-t.svg',
    'animate-elem-07-t.svg',
    'animate-elem-08-t.svg',
    'animate-elem-09-t.svg',
    'animate-elem-10-t.svg',
    'animate-elem-11-t.svg',
    'animate-elem-12-t.svg',
    'animate-elem-13-t.svg',
    'animate-elem-14-t.svg',
    'animate-elem-15-t.svg',
    'animate-elem-17-t.svg',
    'animate-elem-19-t.svg',
    'animate-elem-20-t.svg',
    'animate-elem-21-t.svg',
    'animate-elem-22-b.svg',
    'animate-elem-23-t.svg',
    'animate-elem-24-t.svg',
    'animate-elem-25-t.svg',
    'animate-elem-26-t.svg',
    'animate-elem-27-t.svg',
    'animate-elem-28-t.svg',
    'animate-elem-29-b.svg',
    'animate-elem-30-t.svg',
    'animate-elem-31-t.svg',
    'animate-elem-32-t.svg',
    'animate-elem-33-t.svg',
    'animate-elem-34-t.svg',
    'animate-elem-35-t.svg',
    'animate-elem-36-t.svg',
    'animate-elem-37-t.svg',
    'animate-elem-38-t.svg',
    'animate-elem-39-t.svg',
    'animate-elem-40-t.svg',
    'animate-elem-41-t.svg',
    'animate-elem-44-t.svg',
    'animate-elem-46-t.svg',
    'animate-elem-52-t.svg',
    'animate-elem-53-t.svg',
    'animate-elem-60-t.svg',
    'animate-elem-61-t.svg',
    'animate-elem-62-t.svg',
    'animate-elem-63-t.svg',
    'animate-elem-64-t.svg',
    'animate-elem-65-t.svg',
    'animate-elem-66-t.svg',
    'animate-elem-67-t.svg',
    'animate-elem-68-t.svg',
    'animate-elem-69-t.svg',
    'animate-elem-70-t.svg',
    'animate-elem-77-t.svg',
    'animate-elem-78-t.svg',
    'animate-elem-80-t.svg',
    'animate-elem-81-t.svg',
    'animate-elem-82-t.svg',
    'animate-elem-83-t.svg',
    'animate-elem-84-t.svg',
    'animate-elem-85-t.svg',
    'animate-elem-86-t.svg',
    'animate-elem-87-t.svg',
    'animate-elem-88-t.svg',
    'animate-elem-89-t.svg',
    'animate-elem-90-b.svg',
    'animate-elem-91-t.svg',
    'animate-elem-92-t.svg',
    'animate-interact-events-01-t.svg',
    'animate-interact-pevents-01-t.svg',
    'animate-interact-pevents-02-t.svg',
    'animate-interact-pevents-03-t.svg',
    'animate-interact-pevents-04-t.svg',
    'animate-pservers-grad-01-b.svg',
    'animate-script-elem-01-b.svg',
    'animate-struct-dom-01-b.svg',
    'color-prop-04-t.svg',
    'conform-viewers-03-f.svg',
    'coords-dom-01-f.svg',
    'coords-dom-02-f.svg',
    'coords-dom-03-f.svg',
    'coords-dom-04-f.svg',
    'extend-namespace-01-f.svg',
    'filters-conv-03-f.svg',
    'fonts-desc-01-t.svg',
    'fonts-desc-02-t.svg',
    'fonts-desc-03-t.svg',
    'fonts-desc-04-t.svg',
    'fonts-desc-05-t.svg',
    'fonts-elem-01-t.svg',
    'fonts-elem-02-t.svg',
    'fonts-elem-03-b.svg',
    'fonts-elem-04-b.svg',
    'fonts-elem-05-t.svg',
    'fonts-elem-06-t.svg',
    'fonts-elem-07-b.svg',
    'fonts-glyph-02-t.svg',
    'fonts-glyph-03-t.svg',
    'fonts-glyph-04-t.svg',
    'fonts-kern-01-t.svg',
    'fonts-overview-201-t.svg',
    'interact-cursor-01-f.svg',
    'interact-dom-01-b.svg',
    'interact-events-01-b.svg',
    'interact-events-02-b.svg',
    'interact-events-202-f.svg',
    'interact-events-203-t.svg',
    'interact-order-01-b.svg',
    'interact-order-02-b.svg',
    'interact-order-03-b.svg',
    'interact-pevents-01-b.svg',
    'interact-pevents-03-b.svg',
    'interact-pevents-04-t.svg',
    'interact-pevents-05-b.svg',
    'interact-pevents-07-t.svg',
    'interact-pevents-08-f.svg',
    'interact-pevents-09-f.svg',
    'interact-pevents-10-f.svg',
    'interact-pointer-01-t.svg',
    'interact-pointer-02-t.svg',
    'interact-pointer-03-t.svg',
    'interact-pointer-04-f.svg',
    'interact-zoom-01-t.svg',
    'interact-zoom-02-t.svg',
    'interact-zoom-03-t.svg',
    'masking-path-09-b.svg',
    'masking-path-12-f.svg',
    'paths-dom-01-f.svg',
    'paths-dom-02-f.svg',
    'script-handle-01-b.svg',
    'script-handle-02-b.svg',
    'script-handle-03-b.svg',
    'script-handle-04-b.svg',
    'script-specify-01-f.svg',
    'script-specify-02-f.svg',
    'struct-dom-01-b.svg',
    'struct-dom-02-b.svg',
    'struct-dom-03-b.svg',
    'struct-dom-04-b.svg',
    'struct-dom-05-b.svg',
    'struct-dom-06-b.svg',
    'struct-dom-07-f.svg',
    'struct-dom-08-f.svg',
    'struct-dom-11-f.svg',
    'struct-dom-12-b.svg',
    'struct-dom-13-f.svg',
    'struct-dom-14-f.svg',
    'struct-dom-15-f.svg',
    'struct-dom-16-f.svg',
    'struct-dom-17-f.svg',
    'struct-dom-18-f.svg',
    'struct-dom-19-f.svg',
    'struct-dom-20-f.svg',
    'struct-image-11-b.svg',
    'struct-svg-01-f.svg',
    'struct-svg-02-f.svg',
    'struct-use-14-f.svg',
    'struct-use-15-f.svg',
    'styling-css-06-b.svg',
    'styling-pres-02-f.svg',
    'svgdom-over-01-f.svg',
    'text-altglyph-01-b.svg',
    'text-altglyph-02-b.svg',
    'text-altglyph-03-b.svg',
    'text-dom-01-f.svg',
    'text-dom-02-f.svg',
    'text-dom-03-f.svg',
    'text-dom-04-f.svg',
    'text-dom-05-f.svg',
    'text-tselect-01-b.svg',
    'text-tselect-02-f.svg',
    'text-tselect-03-f.svg',
    'types-dom-01-b.svg',
    'types-dom-02-f.svg',
    'types-dom-03-b.svg',
    'types-dom-04-b.svg',
    'types-dom-05-b.svg',
    'types-dom-06-f.svg',
    'types-dom-07-f.svg',
    'types-dom-08-f.svg',
    'types-dom-svgfittoviewbox-01-f.svg',
    'types-dom-svglengthlist-01-f.svg',
    'types-dom-svgnumberlist-01-f.svg',
    'types-dom-svgstringlist-01-f.svg',
    'types-dom-svgtransformable-01-f.svg',
]

tests_url = 'https://www.w3.org/Graphics/SVG/Test/20110816/archives/W3C_SVG_11_TestSuite.tar.gz'
tests_archive = 'W3C_SVG_11_TestSuite.tar.gz'
tests_dir = 'test_suite'

if os.path.isdir(tests_dir):
    print('Nothing to do.')
    exit()

if not os.path.isfile(tests_archive):
    urllib.request.urlretrieve(tests_url, 'W3C_SVG_11_TestSuite.tar.gz')

with tarfile.open(tests_archive, "r:gz") as tar:
    tar.extractall(tests_dir)

shutil.rmtree(tests_dir + '/harness')
shutil.rmtree(tests_dir + '/png')
shutil.rmtree(tests_dir + '/svgweb')

for test in out_of_scope_tests:
    try:
        os.remove(tests_dir + '/svg/' + test)
    except OSError:
        pass
