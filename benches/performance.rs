//! Performance benchmarks for Lucent
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use lucent::{Scanner, FileMetrics, ProjectMetrics};
use std::path::Path;

fn bench_scanner(c: &mut Criterion) {
    let mut group = c.benchmark_group("scanner");

    group.bench_function("scan_src_directory", |b| {
        b.iter(|| {
            let scanner = Scanner::new(black_box("src")).unwrap();
            let files = scanner.scan().unwrap();
            black_box(files)
        });
    });

    group.finish();
}

fn bench_file_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_analysis");

    // Find a representative Rust file to benchmark
    let test_file = Path::new("src/lib.rs");

    if test_file.exists() {
        group.bench_function("analyze_lib_rs", |b| {
            b.iter(|| {
                if let Some(lang) = lucent::Language::from_path(test_file) {
                    let metrics = FileMetrics::analyze(black_box(test_file), lang).unwrap();
                    black_box(metrics)
                }
            });
        });
    }

    group.finish();
}

fn bench_project_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("project_analysis");

    group.bench_function("analyze_full_project", |b| {
        b.iter(|| {
            let scanner = Scanner::new(black_box("src")).unwrap();
            let files = scanner.scan().unwrap();

            let mut project_metrics = ProjectMetrics::new();
            for source_file in &files {
                if let Ok(metrics) = FileMetrics::analyze(&source_file.path, source_file.language) {
                    project_metrics.add_file(source_file.language.name, &metrics);
                }
            }
            black_box(project_metrics)
        });
    });

    group.finish();
}

fn bench_parallel_vs_sequential(c: &mut Criterion) {
    use rayon::prelude::*;

    let mut group = c.benchmark_group("parallel_comparison");

    // Get files once for reuse
    let scanner = Scanner::new("src").unwrap();
    let files = scanner.scan().unwrap();

    group.bench_function("sequential_analysis", |b| {
        b.iter(|| {
            let mut project_metrics = ProjectMetrics::new();
            for source_file in &files {
                if let Ok(metrics) = FileMetrics::analyze(&source_file.path, source_file.language) {
                    project_metrics.add_file(source_file.language.name, &metrics);
                }
            }
            black_box(project_metrics)
        });
    });

    group.bench_function("parallel_analysis", |b| {
        b.iter(|| {
            let results: Vec<_> = files.par_iter()
                .filter_map(|source_file| {
                    FileMetrics::analyze(&source_file.path, source_file.language)
                        .ok()
                        .map(|m| (source_file.language.name, m))
                })
                .collect();

            let mut project_metrics = ProjectMetrics::new();
            for (lang_name, metrics) in results {
                project_metrics.add_file(lang_name, &metrics);
            }
            black_box(project_metrics)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_scanner,
    bench_file_analysis,
    bench_project_analysis,
    bench_parallel_vs_sequential
);
criterion_main!(benches);
