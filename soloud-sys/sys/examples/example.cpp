// g++ -I ../soloud/include example.cpp ../../../target/debug/build/soloud-sys-69e1b085f8a6624a/out/lib/libsoloud.a

#include <stdio.h>

#include "soloud.h"
#include "soloud_wav.h"
#include "soloud_thread.h"

int main() {
	SoLoud::Soloud sl;
    SoLoud::Wav wav;
	sl.init();

    auto ret = wav.load("覆.mp3");
    printf("%d\n", ret);

	sl.play(wav);

	while (sl.getActiveVoiceCount() > 0) {
		SoLoud::Thread::sleep(100);
	}

	sl.deinit();

	return 0;
}