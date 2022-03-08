DOT_DIR = $$(pwd)/test-conf.d
HOME_DIR = $$(pwd)/test-home

RESET = \033[0m
RED = \033[31m
GREEN = \033[32m
YELLOW = \033[33m
BLUE = \033[34m

# Linking test for dot files
test:
	@echo "Linking test for between $(BLUE)$(DOT_DIR)$(RESET) and $(BLUE)$(HOME_DIR)$(RESET)\n"
	@for file in $$(\ls -A $(DOT_DIR) | grep -v .DS_Store); do \
		if [[ -d $(HOME_DIR)/$${file} ]]; then \
			for file2 in $$(\ls -A $(DOT_DIR)/$${file}); do \
				test "$$(readlink $(HOME_DIR)/$${file}/$${file2})" = "$(DOT_DIR)/$${file}/$${file2}" && echo "$(GREEN)✔︎$(RESET) $${file}/$${file2}" || echo "$(RED)✖︎$(RESET) $${file}/$${file2}"; \
			done \
		else \
			test "$$(readlink $(HOME_DIR)/$${file})" = "$(DOT_DIR)/$${file}" && echo "$(GREEN)✔︎$(RESET) $${file}" || echo "$(RED)✖︎$(RESET) $${file}"; \
		fi \
	done | column -t
.PHONY: test

# List dot files
ls:
	@\ls -A $(DOT_DIR)
.PHONY: ls

# Link dot files
link:
	@for file in $$(\ls -A $(DOT_DIR) | $${FILTER:=fzf} -m); do \
		[ -d $(DOT_DIR)/.config -a ! -e $(HOME_DIR)/.config ] && mkdir $(HOME_DIR)/.config; \
		if [[ -d $(HOME_DIR)/$${file} ]]; then \
			echo "$(GREEN)✔︎$(RESET) $(YELLOW)$${file}$(RESET)"; \
			for file2 in $$(\ls -A $(DOT_DIR)/$${file}); do \
				test -e $(HOME_DIR)/$${file}/$${file2} && echo "$(GREEN)✔︎$(RESET) $(YELLOW)$${file}/$${file2}$(RESET) is already exists" || (ln -s $(DOT_DIR)/$${file}/$${file2} $(HOME_DIR)/$${file}/$${file2} && echo "Create link for $${file}/$${file2}"); \
			done \
		else \
			test -e $(HOME_DIR)/$${file} && echo "$(GREEN)✔︎$(RESET) $(YELLOW)$${file}$(RESET) is already exists" || ln -s $(DOT_DIR)/$${file} $(HOME_DIR)/$${file}; \
		fi \
	done
.PHONY: link

# Link all dot files
linkall:
	@for file in $$(\ls -A $(DOT_DIR) ); do \
		[ -d $(DOT_DIR)/.config -a ! -e $(HOME_DIR)/.config ] && mkdir $(HOME_DIR)/.config; \
		if [[ -d $(HOME_DIR)/$${file} ]]; then \
			echo "$(GREEN)✔︎$(RESET) $(YELLOW)$${file}$(RESET)"; \
			for file2 in $$(\ls -A $(DOT_DIR)/$${file}); do \
				test -e $(HOME_DIR)/$${file}/$${file2} && echo "$(GREEN)✔︎$(RESET) $(YELLOW)$${file}/$${file2}$(RESET) is already exists" || (ln -s $(DOT_DIR)/$${file}/$${file2} $(HOME_DIR)/$${file}/$${file2} && echo "Create link for $${file}/$${file2}"); \
			done \
		else \
			test -e $(HOME_DIR)/$${file} && echo "$(GREEN)✔︎$(RESET) $(YELLOW)$${file}$(RESET) is already exists" || ln -s $(DOT_DIR)/$${file} $(HOME_DIR)/$${file}; \
		fi \
	done
.PHONY: linkall

# Unlink dot files
unlink:
	@for file in $$(\ls -A $(DOT_DIR) | $${FILTER:=fzf} -m); do \
		[ -d $(DOT_DIR)/.config -a ! -e ${HOME}/.config ] && mkdir ${HOME}/.config; \
		if [[ -d $(HOME_DIR)/$${file} ]]; then \
			echo "$(GREEN)✔︎$(RESET) $(YELLOW)$${file}$(RESET)"; \
			for file2 in $$(\ls -A $(DOT_DIR)/$${file}); do \
				test -e $(HOME_DIR)/$${file}/$${file2} && unlink $(HOME_DIR)/$${file}/$${file2} && echo "Remove link for $${file}/$${file2}"; \
			done \
		else \
			test -e $(HOME_DIR)/$${file} && unlink $(HOME_DIR)/$${file} && echo "Remove link for $${file}/$${file2}"; \
		fi \
	done
.PHONY: unlink
