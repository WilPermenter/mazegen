# mazegen
Exploring diffrent maze generation Algs, Might work on the reverse at some point and explore solving them.

<img width="1250" height="510" alt="maze" src="https://github.com/user-attachments/assets/5987594c-6f4d-4644-8e73-5c9a0a391d59" />

# How to Use

Currently, the project just calls main.rs when running:

```cargo run```

Look in main.rs to modify the size of the maze.
Currently Supported Algorithms

   Depth-First Search (DFS):
    With depth-first search, we have two options:

  ```generate_dfs_rec``` — Recursive

  ```generate_dfs_stack``` — Stack-Based

   The recursive version is faster but can run into memory stack issues due to deep recursion. Use this if you'd like to generate a lot of mazes quickly.

   The stack-based version uses a Vec to keep track of the current node instead of recursion. This avoids the thread stack bottleneck but adds some overhead when manipulating the vector. It is slightly slower, but can generate very large mazes.
  
