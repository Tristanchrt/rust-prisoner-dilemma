# Directories
FRONT_DIR = front
BACK_DIR = back
SETTINGS_DIR = settings

# Cargo commands
CARGO = cargo
CARGO_BUILD = $(CARGO) build
CARGO_RUN = $(CARGO) run
CARGO_TEST = $(CARGO) test

.PHONY: runf runb runS runTS runTF runTB runAllTest

runf:
	cd $(FRONT_DIR) && $(CARGO_BUILD) && $(CARGO_RUN)

runb:
	cd $(BACK_DIR) && $(CARGO_BUILD) && $(CARGO_RUN)

runS:
	cd $(SETTINGS_DIR) && $(CARGO_BUILD) && $(CARGO_RUN)

runTS:
	cd $(SETTINGS_DIR) && $(CARGO_TEST)

runTF:
	cd $(FRONT_DIR) && $(CARGO_TEST)

runTB:
	cd $(BACK_DIR) && $(CARGO_TEST)

runAllTest: runTS runTF runTB
