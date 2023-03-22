use hydrabadger::Blockchain;

#[test]
fn blockchain() -> Result<(), MiningError>{
    let mut chain = Blockchain::new()?;
    println!("Send 1 Hydradollar to Bob");
    chain.add_block("1HD->Bob")?;
    chain.add_block("0.5HD->Bob")?;
    chain.add_block("1.5HD->Bob")?;

    println!("Traversing blockchain:\n");
    chain.traverse();
    Ok(())
}