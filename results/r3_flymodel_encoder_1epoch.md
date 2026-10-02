# R3 sanity run with a 1-epoch LeNet5 encoder (KMNIST test acc 0.9129); same r3_flymodel code, before the m=3200 control was added
| variant | acc@10 classes | acc@20 classes | mean memory loss | sd acc@10 | sd acc@20 |
|---|---|---|---|---|---|
| flymodel | 0.860 | 0.744 | 0.060 | 0.0022 | 0.0031 |
| abl_dense_coding | 0.098 | 0.049 | 0.000 | 0.0000 | 0.0000 |
| abl_perceptron_v1 | 0.113 | 0.101 | 0.465 | 0.0075 | 0.0162 |
| abl_perceptron_v2 | 0.162 | 0.103 | 0.659 | 0.0067 | 0.0041 |
| abl_perceptron_v3 | 0.183 | 0.122 | 0.640 | 0.0075 | 0.0031 |
| abl_dense_perceptron_v1 | 0.101 | 0.050 | 0.453 | 0.0000 | 0.0000 |
| ctl_raw_pixels_no_encoder | 0.907 | 0.761 | 0.056 | 0.0015 | 0.0022 |
