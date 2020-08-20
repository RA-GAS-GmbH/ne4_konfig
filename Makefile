
.PHONY: release release-ra-gas clean

release:
	cp "package default.sh" package.sh
	cp "Setup default.nsi" Setup.nsi
	docker start -ai ne4_konfig-build

release-ra-gas:
	cp "package ra-gas.sh" package.sh
	cp "Setup ra-gas.nsi" Setup.nsi
	docker start -ai ne4_konfig-build

clean:
	rm *.exe
