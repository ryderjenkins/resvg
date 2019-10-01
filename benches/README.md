## Curves

Rendering a lot of curves:

```
test fill_cairo                                  ... bench:   5,624,373 ns/iter (+/- 56,009)
test fill_qt                                     ... bench:  19,648,004 ns/iter (+/- 101,633)
test fill_raqote                                 ... bench:   2,328,939 ns/iter (+/- 33,038)
test fill_skia                                   ... bench:   3,203,706 ns/iter (+/- 31,534)

test stroke_cairo                                ... bench:  14,013,711 ns/iter (+/- 73,789)
test stroke_qt                                   ... bench:   2,180,796 ns/iter (+/- 27,700)
test stroke_raqote                               ... bench:  11,852,373 ns/iter (+/- 170,112)
test stroke_skia                                 ... bench:   2,569,262 ns/iter (+/- 36,969)

test dashed_stroke_cairo                         ... bench:  13,350,131 ns/iter (+/- 67,628)
test dashed_stroke_qt                            ... bench:   2,064,253 ns/iter (+/- 23,471)
test dashed_stroke_raqote                        ... bench:   7,630,904 ns/iter (+/- 100,983)
test dashed_stroke_skia                          ... bench:   2,978,653 ns/iter (+/- 25,245)

test fill_and_stroke_cairo                       ... bench:  18,955,524 ns/iter (+/- 68,498)
test fill_and_stroke_qt                          ... bench:  20,806,866 ns/iter (+/- 254,622)
test fill_and_stroke_raqote                      ... bench:  13,599,629 ns/iter (+/- 99,935)
test fill_and_stroke_skia                        ... bench:   5,344,305 ns/iter (+/- 34,660)

test fill_and_stroke_with_gradient_cairo         ... bench:  21,306,847 ns/iter (+/- 85,123)
test fill_and_stroke_with_gradient_qt            ... bench:  22,531,950 ns/iter (+/- 237,340)
test fill_and_stroke_with_gradient_raqote        ... bench:  14,539,157 ns/iter (+/- 148,509)
test fill_and_stroke_with_gradient_skia          ... bench:   6,553,852 ns/iter (+/- 28,627)

test fill_and_stroke_with_pattern_cairo          ... bench:  22,100,329 ns/iter (+/- 110,188)
test fill_and_stroke_with_pattern_qt             ... bench:  22,831,600 ns/iter (+/- 210,146)
test fill_and_stroke_with_pattern_raqote         ... bench:  29,812,395 ns/iter (+/- 179,026)
test fill_and_stroke_with_pattern_skia           ... bench:   7,014,667 ns/iter (+/- 31,498)
```

Rendering a circle:

```
test fill_circle_cairo                           ... bench:      29,840 ns/iter (+/- 168)
test fill_circle_qt                              ... bench:      54,496 ns/iter (+/- 704)
test fill_circle_raqote                          ... bench:     125,162 ns/iter (+/- 2,140)
test fill_circle_skia                            ... bench:      28,553 ns/iter (+/- 149)

test stroke_circle_cairo                         ... bench:      72,730 ns/iter (+/- 1,495)
test stroke_circle_qt                            ... bench:      33,939 ns/iter (+/- 803)
test stroke_circle_raqote                        ... bench:      64,197 ns/iter (+/- 7,731)
test stroke_circle_skia                          ... bench:      24,214 ns/iter (+/- 650)

test fill_and_stroke_circle_with_gradient_cairo  ... bench:     457,841 ns/iter (+/- 6,185)
test fill_and_stroke_circle_with_gradient_qt     ... bench:      82,629 ns/iter (+/- 847)
test fill_and_stroke_circle_with_gradient_raqote ... bench:     290,416 ns/iter (+/- 14,014)
test fill_and_stroke_circle_with_gradient_skia   ... bench:      79,784 ns/iter (+/- 324)

test fill_and_stroke_circle_with_pattern_cairo   ... bench:     292,270 ns/iter (+/- 2,007)
test fill_and_stroke_circle_with_pattern_qt      ... bench:      83,025 ns/iter (+/- 829)
test fill_and_stroke_circle_with_pattern_raqote  ... bench:   1,599,755 ns/iter (+/- 9,840)
test fill_and_stroke_circle_with_pattern_skia    ... bench:     118,654 ns/iter (+/- 1,381)
```

## Filters

```
test blend_multiply_cairo        ... bench:   1,031,263 ns/iter (+/- 22,750)
test blend_multiply_qt           ... bench:     529,434 ns/iter (+/- 13,601)
test blend_multiply_raqote       ... bench:   1,571,461 ns/iter (+/- 27,929)
test blend_multiply_skia         ... bench:     559,557 ns/iter (+/- 7,449)

test box_blur_100px              ... bench:     489,889 ns/iter (+/- 11,179)
test iir_blur_100px              ... bench:   2,103,825 ns/iter (+/- 9,360)

test box_blur_500px              ... bench:  11,266,742 ns/iter (+/- 60,679)
test iir_blur_500px              ... bench:  52,655,911 ns/iter (+/- 130,209)

test color_matrix_cairo          ... bench:     968,304 ns/iter (+/- 13,222)
test color_matrix_qt             ... bench:     531,380 ns/iter (+/- 7,901)
test color_matrix_raqote         ... bench:   1,277,710 ns/iter (+/- 16,325)
test color_matrix_skia           ... bench:     660,396 ns/iter (+/- 7,727)

test composite_over_cairo        ... bench:     670,280 ns/iter (+/- 18,277)
test composite_over_qt           ... bench:     482,695 ns/iter (+/- 9,206)
test composite_over_raqote       ... bench:   1,531,026 ns/iter (+/- 20,436)
test composite_over_skia         ... bench:     532,380 ns/iter (+/- 7,600)

test composite_arithmetic_cairo  ... bench:   1,064,570 ns/iter (+/- 10,764)
test composite_arithmetic_qt     ... bench:   1,052,673 ns/iter (+/- 9,157)
test composite_arithmetic_raqote ... bench:   1,570,792 ns/iter (+/- 18,028)
test composite_arithmetic_skia   ... bench:   1,201,947 ns/iter (+/- 11,322)
```

## Layers

```
test element_with_opacity_cairo  ... bench:      79,572 ns/iter (+/- 3,549)
test element_with_opacity_qt     ... bench:      37,945 ns/iter (+/- 414)
test element_with_opacity_raqote ... bench:     189,576 ns/iter (+/- 3,080)
test element_with_opacity_skia   ... bench:      46,417 ns/iter (+/- 599)

test groups_with_opacity_cairo   ... bench:     107,714 ns/iter (+/- 3,491)
test groups_with_opacity_qt      ... bench:      73,639 ns/iter (+/- 1,225)
test groups_with_opacity_raqote  ... bench:     345,406 ns/iter (+/- 11,096)
test groups_with_opacity_skia    ... bench:      88,536 ns/iter (+/- 849)

test clip_path_cairo             ... bench:      71,351 ns/iter (+/- 996)
test clip_path_qt                ... bench:     208,616 ns/iter (+/- 3,394)
test clip_path_raqote            ... bench:     436,002 ns/iter (+/- 5,728)
test clip_path_skia              ... bench:     104,175 ns/iter (+/- 1,547)

test nested_clip_path_cairo      ... bench:     114,061 ns/iter (+/- 2,314)
test nested_clip_path_qt         ... bench:     282,545 ns/iter (+/- 3,741)
test nested_clip_path_raqote     ... bench:     597,315 ns/iter (+/- 20,595)
test nested_clip_path_skia       ... bench:     162,694 ns/iter (+/- 1,185)

test mask_cairo                  ... bench:      83,569 ns/iter (+/- 4,901)
test mask_qt                     ... bench:     182,965 ns/iter (+/- 6,062)
test mask_raqote                 ... bench:     651,327 ns/iter (+/- 15,739)
test mask_skia                   ... bench:     201,844 ns/iter (+/- 3,772)
```
