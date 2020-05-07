use cc;

fn main() {
    let src_files = vec![
        "libprimesieve/src/api-c.cpp",
        "libprimesieve/src/api.cpp",
        "libprimesieve/src/CpuInfo.cpp",
        "libprimesieve/src/EratBig.cpp",
        "libprimesieve/src/EratMedium.cpp",
        "libprimesieve/src/EratSmall.cpp",
        "libprimesieve/src/iterator-c.cpp",
        "libprimesieve/src/iterator.cpp",
        "libprimesieve/src/IteratorHelper.cpp",
        "libprimesieve/src/LookupTables.cpp",
        "libprimesieve/src/MemoryPool.cpp",
        "libprimesieve/src/PrimeGenerator.cpp",
        "libprimesieve/src/nthPrime.cpp",
        "libprimesieve/src/ParallelSieve.cpp",
        "libprimesieve/src/popcount.cpp",
        "libprimesieve/src/PreSieve.cpp",
        "libprimesieve/src/PrintPrimes.cpp",
        "libprimesieve/src/PrimeSieve.cpp",
        "libprimesieve/src/Erat.cpp",
        "libprimesieve/src/SievingPrimes.cpp",
        "primesieve-wa.c",
    ];

    cc::Build::new()
        .cpp(true)
        .flag("-march=native")
        .opt_level(3)
        .warnings(false)
        .files(src_files)
        .include("libprimesieve/include")
        .compile("primesieve");

}
