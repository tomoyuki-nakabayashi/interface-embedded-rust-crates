use heapless::{spsc, box_pool, pool::boxed::{Box, BoxBlock}};
use std::thread;

// 最大2048バイト長を扱えるパケットバッファ
type PacketBuffer = heapless::Vec<u8, 2048>;
// `PacketBuffer`のメモリプール`P`を作成
box_pool!(P: PacketBuffer);

fn main() {
    // `Box<P>`を送受信するためのキューを作成
    let q: &'static mut spsc::Queue::<Box<P>, 8> = {
        static mut Q: spsc::Queue::<Box<P>, 8> = spsc::Queue::<Box<P>, 8>::new();
        unsafe { &mut Q }
    };
    let (mut producer, mut consumer) = q.split();

    thread::spawn(move || {
        // パケットバッファ6個分のメモリブロックを静的に確保
        let blocks: &'static mut [BoxBlock<PacketBuffer>] = {
            const BLOCK: BoxBlock<PacketBuffer> = BoxBlock::new();
            static mut BLOCKS: [BoxBlock<PacketBuffer>; 6] = [BLOCK; 6];
            unsafe { &mut BLOCKS }
        };
        // 確保したメモリブロックを`P`のメモリプールに登録
        // 以降、最大で6個までのパケットバッファをメモリプール`P`から割り当てできる
        for block in blocks {
            P.manage(block);
        }

        let mut sent = 0;
        loop {
            // 1秒ごとにメモリプール`P`からパケットバッファを割り当てて、パケットをメインスレッドに送信
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

    // 10秒間パケットバッファを受け取らずにいると、途中からパケットバッファの割り当てができなくなる
    thread::sleep(std::time::Duration::from_millis(10_000));

    loop {
        let _ = consumer.dequeue()
            .map(|packet| {
                println!("received: {:?}", packet);
                // スコープを抜けるとパケットバッファ (packet) が自動で解放される。解放は順不同でも良い
                // パケットバッファが解放されると、メモリプール`P`に再利用可能なメモリブロックが戻り、再び割り当て可能になる
            });
        thread::sleep(std::time::Duration::from_millis(10));
    }
}
