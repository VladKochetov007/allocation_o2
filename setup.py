from setuptools import setup, find_packages

setup(
    name="allocation_o2",
    version="0.1.0",
    description="Tactical asset allocation library with Rust backend",
    author="Vlad Kochetov",
    author_email="vladkoch@knu.ua",
    packages=find_packages(include=["allocation_o2", "allocation_o2.*"]),
    package_data={
        "allocation_o2": ["*.so"],
    },
    include_package_data=True,
    python_requires=">=3.8",
    install_requires=[
        "numpy>=1.20.0",
        "matplotlib>=3.5.0",
    ],
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Intended Audience :: Financial and Insurance Industry",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Rust",
        "Topic :: Scientific/Engineering :: Mathematics",
        "Topic :: Office/Business :: Financial :: Investment",
    ],
) 