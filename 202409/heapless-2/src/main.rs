use heapless::{spsc, box_pool, pool::boxed::{Box, BoxBlock}};
use std::thread;

type PacketBuffer = heapless::Vec<u8, 2048>;
box_pool!(P: PacketBuffer);

fn main() {
    let q: &'static mut spsc::Queue::<Box<P>, 8> = {
        static mut Q: spsc::Queue::<Box<P>, 8> = spsc::Queue::<Box<P>, 8>::new();
        unsafe { &mut Q }
    };
    let (mut producer, mut consumer) = q.split();

    thread::spawn(move || {
        let blocks: &'static mut [BoxBlock<PacketBuffer>] = {
            const BLOCK: BoxBlock<PacketBuffer> = BoxBlock::new();
            static mut BLOCKS: [BoxBlock<PacketBuffer>; 6] = [BLOCK; 6];
            unsafe { &mut BLOCKS }
        };
        for block in blocks {
            P.manage(block);
        }

        let mut sent = 0;
        loop {
            let _ = P.alloc(PacketBuffer::new())
                .inspect_err(|_| println!("alloc failed"))
                .map(|mut packet| {
                    // 実際はハードウェアから読み込むなどして packet にデータを書き込む
                    // このサンプルでは何個目のパケットかを書き込むだけ
                    packet.push(sent).ok();
                    producer.enqueue(packet).ok();
                    println!("send packet: {}", sent);
                    sent += 1;
                });
            thread::sleep(std::time::Duration::from_millis(1_000));
        }
    });

    thread::sleep(std::time::Duration::from_millis(10_000));

    loop {
        let _ = consumer.dequeue()
            .map(|packet| {
                println!("received: {:?}", packet);
                // スコープを抜けると packet が自動で解放される
            });
        thread::sleep(std::time::Duration::from_millis(10));
    }
}
