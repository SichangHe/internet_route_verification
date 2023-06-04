#![allow(clippy::type_complexity)]

use std::{
    process::Command,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{spawn, JoinHandle},
};

use anyhow::Result;
use log::debug;

use crate::{
    cmd::PipedChild,
    lex::{
        lines::RPSLObject,
        rpsl_object::{AutNum, PeeringSet},
    },
    serde::from_str,
};

use super::read_line_wait;

pub fn spawn_aut_num_worker() -> Result<(Sender<RPSLObject>, JoinHandle<Result<Vec<AutNum>>>)> {
    let (send, recv) = channel();
    let worker = spawn(|| aut_num_worker(recv));
    Ok((send, worker))
}

fn aut_num_worker(recv: Receiver<RPSLObject>) -> Result<Vec<AutNum>> {
    let mut aut_num_child =
        PipedChild::new(Command::new("pypy3").args(["-m", "rpsl_policy.aut_num"]))?;

    let mut aut_nums = Vec::new();
    while let Ok(obj) = recv.recv() {
        obj.write_to(&mut aut_num_child.stdin)?;
        let line = read_line_wait(&mut aut_num_child.stdout)?;
        let mut aut_num: AutNum = from_str(&line)?;
        (aut_num.name, aut_num.body) = (obj.name, obj.body);
        aut_nums.push(aut_num);
        match aut_nums.len() {
            l if l % 0xFFF == 0 => debug!("Parsed {l} aut_nums."),
            _ => (),
        }
    }
    Ok(aut_nums)
}

pub fn spawn_peering_set_worker(
) -> Result<(Sender<RPSLObject>, JoinHandle<Result<Vec<PeeringSet>>>)> {
    let (send, recv) = channel();
    let worker = spawn(|| peering_set_worker(recv));
    Ok((send, worker))
}

fn peering_set_worker(recv: Receiver<RPSLObject>) -> Result<Vec<PeeringSet>> {
    let mut peering_set_child =
        PipedChild::new(Command::new("pypy3").args(["-m", "rpsl_policy.peering_set"]))?;

    let mut peering_sets = Vec::new();
    while let Ok(obj) = recv.recv() {
        obj.write_to(&mut peering_set_child.stdin)?;
        let line = read_line_wait(&mut peering_set_child.stdout)?;
        let mut peering_set: PeeringSet = from_str(&line)?;
        (peering_set.name, peering_set.body) = (obj.name, obj.body);
        peering_sets.push(peering_set);
        match peering_sets.len() {
            l if l % 0xFF == 0 => debug!("Parsed {l} peering_sets."),
            _ => (),
        }
    }
    Ok(peering_sets)
}
