# rust-algorithm

Rust 实现链表等数据结构

## 已经完成的算法

- [x] 链表
- [x] 双向链表
- [x] 图
- [x] 前缀树

## 技术取舍&思考过程

### Arena 技术

思路：使用 ArenaList，把节点用 Array 管理起来，它类似 对象池

相比 `Rc` 或 `Box`，好处是：

- 数据在内存中连续存储，极大提高了某些操作的性能，同时防止产生大量内存碎片
- 不用每次生成一个节点都生成一个对象，也不用每次释放一个节点都要递归调用 `Drop` 方法
- 杜绝了 `unsafe`，提高安全性

此方案也带来了一些麻烦：

**麻烦1:** 诸如 “合并两个 tree” 的算法会变得复杂，你需要考虑合并前的对象池是否是同一个。

- 这里的使用 **全局唯一** 对象池
- 因为否则的话，链表/树/图的 “合并”/“拆分” 等操作，都需要对象池的合并/拆分，复杂度从 O(1) 变成 O(n)

**麻烦2:** 1个节点对象销毁时，会在连续内存中形成空洞。有频繁增删的场景必须处理空洞，否则消耗内存快速增大。方法有这些：

- **方法1:** Lazy Compaction. 维护一个 `holes: Vec<usize>`，用来记录空洞的序号，程序运行一段时间后，开启 `compact`
  ，消除空洞，压紧内存。
    - `compact` 过程：把最后一个节点与空洞互换，同时调整其它节点的与它之间的指向关系。循环此操作，直到没有空洞。
    - 优点1:删除一个节点，只需要置 None
    - 优点2:新增一个节点，只需加在尾部，没有额外操作
    - 缺点1: 何时开启 `compact` 是个问题
    - 缺点2：`compact` 时，对于每个与空洞互换的节点，都需要寻找每个节点的上游，复杂度为 O(n),m个空洞复杂度就是 O(mn)
- **方法1.1:** 改进方法1，引入 `prev: Vec<usize>`（实际上把链表变成了双向链表），可以使复杂度降低 O(m)，缺点是需要的边翻倍，额外消耗内存、CPU
- **方法2：**：在空洞可能产生的地方（删除节点），立即处理空洞
    - 优点1:压根不会产生任何空洞，也就不需要变量 `holes: Vec<usize>`，并且 `node: Vec<Option<Node<T>>>` 可以改为
      `node: Vec<Node<T>>`
    - 缺点1：每次删除节点，都需要额外做一次交换 swap，和4次指向的变更
- **方法3：** 不处理空洞，而是在之后新建节点时，把新建的节点放到空洞处
    - 空洞可以用单独的链表来管理，类似 `free list`

一些约定

1. `ArenaList.nodes: Vec<Option<T>>` 用来存放数据对象
2. `nexts: Vec<Option<usize>>` 用来存放下一个节点的索引。若为 None，表示没有下一个节点。
3. 维护一个 `holes: Vec<usize>` 用来存放运行过程中产生的孔洞。代码里的 `holes` 用 **stack** 的方式还使用，也可以用 **queue
   ** 的方式来使用（需要换成别的数据结构以保证性能）
4. 统一使用带 dummy 的节点，可以降低代码复杂性。
5. `LinkedList.root: unsize` 代表链表 dummy 节点位置。已有节点上生成另一个链表，只需要 new 一个新的 LinkedList。

ArenaList 数据类型的取舍

- 可以是 `nodes: Vec<Option<T>>` + `nexts: Vec<Option<usize>>`
- 也可以是 `nodes: Vec<NodeInfo<T>>`，其中 `NodeInfo{data, next_idx}`
- 应该没有优劣，我实现的是前者
