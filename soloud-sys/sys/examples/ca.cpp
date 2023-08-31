// g++ -I ../soloud/include example.cpp ../../../target/debug/build/soloud-sys-69e1b085f8a6624a/out/lib/libsoloud.a

#include <stdio.h>

#include "soloud.h"
#include "soloud_wav.h"
#include "soloud_thread.h"
#include <fstream>
#include <vector>

constexpr unsigned long BEEP_INTERVAL = 100000; //milliseconds                                                    
constexpr unsigned long  POLL_INTERVAL = 1000; 

int main() {
    for (int i = 0; i < 5; i++) {
        SoLoud::Soloud sl;
        SoLoud::Wav wav;
        sl.init();
        std::ifstream in("../../../beep.mp3", ::std::ios::binary);
        std::vector<unsigned char> buffer(std::istreambuf_iterator<char>(in), {});
        auto ret = wav.loadMem(buffer.data(), buffer.size(), true, false);

        sl.play(wav);

        while (sl.getVoiceCount() > 0) {
            SoLoud::Thread::sleep(POLL_INTERVAL);
        }

        sl.deinit();
        SoLoud::Thread::sleep(BEEP_INTERVAL);
    }
    
	return 0;
}
