ROOT_DIR = $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

TARGET = $(ROOT_DIR)/target/release/ray-tracer
DBGTARGET = $(ROOT_DIR)/target/debug/ray-tracer

INPDIR = ./scenes
BASE_EXAMPLES = $(wildcard $(INPDIR)/example*.xml)
CUSTOM_EXAMPLES = $(INPDIR)/animation.xml $(INPDIR)/spotlight.xml $(INPDIR)/cook-torrance.xml $(INPDIR)/depth_of_field.xml $(INPDIR)/julia_animated.xml $(INPDIR)/julia_set.xml

.PHONY: release debug-target all debug chess

release:
	cargo build --release

debug-target:
	cargo build

all: release
	@- echo "Tracing base scenes."
	@- $(foreach SCENE, $(BASE_EXAMPLES), \
			echo ""; \
			$(TARGET) $(SCENE) -p; \
			echo ""; \
		)
	@- echo "Tracing custom scenes. This might take a while..."
	@- $(foreach SCENE, $(CUSTOM_EXAMPLES), \
			echo ""; \
			$(TARGET) $(SCENE) -p; \
			echo ""; \
		)

debug: debug-target
	@- echo "Tracing base scenes."
	@- $(foreach SCENE, $(BASE_EXAMPLES), \
			echo ""; \
			$(DBGTARGET) $(SCENE) -p; \
			echo ""; \
		)

chess: release
	@- echo "Tracing chess scene. This might take a while..."
	@- echo ""
	@- $(TARGET) $(INPDIR)/chess.xml -p
