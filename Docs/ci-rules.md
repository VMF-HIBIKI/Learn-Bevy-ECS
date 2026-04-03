# CI 与协作规则

本文档定义本仓库的 CI 要求和协作约束。下文中的 Merge Request 在 GitHub 中对应 Pull Request（PR）。

## 目标

- `master` 是唯一主分支。
- 任何改动都不能直接提交到 `master`。
- 任何 PR 都必须先从一个 Issue 开始创建。
- Issue、Commit 和 CI 结果需要形成完整的可追溯链路。

## 分支与合并策略

1. `master` 仅接收通过 CI 的 PR。
2. 不允许绕过 PR 直接向 `master` 推送代码。
3. 新需求、修复、重构或文档更新都必须先创建 Issue。
4. 创建 Issue 后，使用 GitHub 的 Issue 创建分支能力，从该 Issue 派生工作分支，再提交 PR 到 `master`。

## Issue 规范

Issue 标题必须符合以下格式：

```text
<type>: <short summary>
```

允许的 `type` 只有：

```text
build | ci | docs | feat | fix | perf | refactor | test | demo | bench
```

示例：

```text
ci: add nightly lint workflow
docs: describe contribution flow
feat: add basic ecs entity module
```

每个 Issue 都必须至少分配给 1 个 assignee。

## Commit 规范

Commit 标题必须符合以下格式：

```text
<type>: <short summary> (#<issue ID>)
```

示例：

```text
ci: add nightly fmt and clippy workflow (#12)
docs: document master branch policy (#13)
```

这条规则用于确保每次提交都能回溯到对应的 Issue。

补充说明：

- GitHub 不支持给 commit 单独设置 assignee。
- 因此本仓库将“commit 需要 assign 一个人”解释为：commit 必须关联一个 Issue，且该 Issue 必须已经分配给至少 1 个 assignee。
- 每个 Issue 只能对应 1 个非 merge commit。
- 如果需要修复已存在的 commit，必须使用 `git commit --amend` 后再 force push 到工作分支，而不是继续追加新的 commit。
- Commit 尾部的 `#<issue ID>` 必须与工作分支名前缀中的 issue ID 一致。

## PR 规范

1. PR 的目标分支必须是 `master`。
2. PR 对应的工作分支应从 Issue 创建，建议分支名以 Issue 编号开头，例如 `12-ci-workflow`。
3. 每个 PR 只能包含 1 个非 merge commit。
4. 若提交后发现问题，必须在原 commit 上 `amend`，而不是继续追加新的 commit。
5. PR 合并前，必须通过仓库 CI。
6. PR 必须至少获得 1 次审批后才能合并。
7. PR 必须由 Code Owner 手动审批后才能合并。
8. 当前仓库的 Code Owner 为 `@VMF-HIBIKI`，定义见 [CODEOWNERS](/e:/RustProject/Learn-Bevy-ECS/.github/CODEOWNERS)。
9. 每个 PR 都必须至少分配给 1 个 assignee。

## CI 检查内容

仓库 CI 会执行以下检查：

1. Issue 标题格式检查。
2. Issue assignee 检查。
3. PR 分支命名检查。
4. PR assignee 检查。
5. 单 Issue 单 Commit 检查。
6. Commit 信息与分支 issue ID 一致性检查。
7. Commit 关联 Issue 的 assignee 检查。
8. `cargo +nightly fmt --all -- --check`
9. `cargo +nightly clippy --all-targets --all-features -- -D warnings`

其中 `clippy` 以 `-D warnings` 运行，意味着任何警告都会导致 CI 失败。

## 当前 GitHub 规则

当前仓库已经改为使用 repository rulesets 管理 `master`，实际生效规则如下：

1. `master` 必须通过 `CI / policy`。
2. `master` 必须通过 `CI / rust-ci`。
3. 启用了严格状态检查，分支落后于 `master` 时需要先更新后再合并。
4. `master` 不允许 force push。
5. `master` 不允许删除。
6. 非管理员更新 `master` 时，必须通过 PR。
7. 非管理员的 PR 必须至少获得 1 次审批。
8. 非管理员的 PR 必须由 Code Owner 审批。
9. Code Owner 当前为 `@VMF-HIBIKI`。

补充说明：

- `master` 的 CI 约束对所有人都生效，包括仓库管理员。
- `@VMF-HIBIKI` 作为管理员，可以绕过 `master` 上的 PR/审批规则，但不能绕过 CI 检查。
- 其他协作者仍然必须通过 PR，并获得 `@VMF-HIBIKI` 的 Code Owner 审批后才能合并。
- 普通功能分支允许 force push，这不受 `master` 规则限制。
