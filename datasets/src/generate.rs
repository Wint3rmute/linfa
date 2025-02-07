//! Utility functions for randomly generating datasets

use ndarray::{s, Array, Array2, ArrayBase, Data, Ix1, Ix2};
use ndarray_rand::rand::Rng;
use ndarray_rand::rand_distr::{Distribution, StandardNormal};
use ndarray_rand::RandomExt;

/// Special case of `blobs_with_distribution` with a standard normal distribution.
pub fn blobs(
    blob_size: usize,
    blob_centroids: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    rng: &mut impl Rng,
) -> Array2<f64> {
    blobs_with_distribution(blob_size, blob_centroids, StandardNormal, rng)
}

/// Given an input matrix `blob_centroids`, with shape `(n_blobs, n_features)`,
/// generate `blob_size` data points (a "blob") around each of the blob centroids.
///
/// More specifically, each blob is formed by `blob_size` points sampled from a distribution
/// centered in the blob centroid.
///
/// `blobs` can be used to quickly assemble a synthetic dataset to test or
/// benchmark various clustering algorithms on a best-case scenario input.
pub fn blobs_with_distribution(
    blob_size: usize,
    blob_centroids: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    distribution: impl Distribution<f64> + Clone,
    rng: &mut impl Rng,
) -> Array2<f64> {
    let (n_centroids, n_features) = blob_centroids.dim();
    let mut blobs: Array2<f64> = Array2::zeros((n_centroids * blob_size, n_features));

    for (blob_index, blob_centroid) in blob_centroids.rows().into_iter().enumerate() {
        let blob = make_blob(blob_size, &blob_centroid, distribution.clone(), rng);

        let indexes = s![blob_index * blob_size..(blob_index + 1) * blob_size, ..];
        blobs.slice_mut(indexes).assign(&blob);
    }
    blobs
}

/// Generate `blob_size` data points (a "blob") around `blob_centroid` using the given distribution.
///
/// `blob` can be used to quickly assemble a synthetic stereotypical cluster.
fn make_blob(
    blob_size: usize,
    blob_centroid: &ArrayBase<impl Data<Elem = f64>, Ix1>,
    distribution: impl Distribution<f64>,
    rng: &mut impl Rng,
) -> Array2<f64> {
    let shape = (blob_size, blob_centroid.len());
    let origin_blob: Array2<f64> = Array::random_using(shape, distribution, rng);
    origin_blob + blob_centroid
}
