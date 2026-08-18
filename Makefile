.PHONY: fmt lint test check

# 格式化整个 workspace
fmt:
	cargo fmt --all

# 只检查格式是否合规（不修改文件）
fmt-check:
	cargo fmt --all -- --check

# Clippy 全目标检查，警告即失败
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# 测试
test:
	cargo test --all-features

# 提交前完整门禁：格式 + lint + 测试
check: fmt-check lint test
