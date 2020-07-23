ODIR=obj
SDIR=src

_OBJ = main
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))

_SRC = main.rs
SRC  = $(patsubst %,$(SDIR)/%,$(_SRC))

# raytracer: $(OBJ)
# 	rustc $< -o $@

$(ODIR)/%: $(SDIR)/%.rs
	rustc $< -o $@

.PHONY: clean

clean:
	rm -f $(ODIR)/*