#![allow(clippy::type_complexity)]

use std::{
    process::Command,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{spawn, JoinHandle},
};

use anyhow::Result;
use log::{debug, error, warn};

use crate::{
    cmd::PipedChild,
    lex::{
        lines::RPSLObject,
        rpsl_object::{AutNum, FilterSet, PeeringSet},
    },
    serialize::from_str,
};

use super::read_line_wait;

pub fn spawn_aut_num_worker() -> Result<(Sender<RPSLObject>, JoinHandle<Result<Vec<AutNum>>>)> {
    let (send, recv) = channel();
    let worker = spawn(|| {
        aut_num_worker(recv).map_err(|e| {
            error!("aut_num_worker: {e:#}.");
            e
        })
    });
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
    warn!("aut_num_worker exiting normally.");
    Ok(aut_nums)
}

pub fn spawn_peering_set_worker(
) -> Result<(Sender<RPSLObject>, JoinHandle<Result<Vec<PeeringSet>>>)> {
    let (send, recv) = channel();
    let worker = spawn(|| {
        peering_set_worker(recv).map_err(|e| {
            error!("peering_set_worker: {e:#}.");
            e
        })
    });
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
    warn!("peering_set_worker exiting normally.");
    Ok(peering_sets)
}

pub fn spawn_filter_set_worker() -> Result<(Sender<RPSLObject>, JoinHandle<Result<Vec<FilterSet>>>)>
{
    let (send, recv) = channel();
    let worker = spawn(|| {
        filter_set_worker(recv).map_err(|e| {
            error!("filter_set_worker: {e:#}.");
            e
        })
    });
    Ok((send, worker))
}

fn filter_set_worker(recv: Receiver<RPSLObject>) -> Result<Vec<FilterSet>> {
    let mut filter_set_child =
        PipedChild::new(Command::new("pypy3").args(["-m", "rpsl_policy.filter_set"]))?;

    let mut filter_sets = Vec::new();
    while let Ok(obj) = recv.recv() {
        obj.write_to(&mut filter_set_child.stdin)?;
        let line = read_line_wait(&mut filter_set_child.stdout)?;
        let mut filter_set: FilterSet = from_str(&line)?;
        (filter_set.name, filter_set.body) = (obj.name, obj.body);
        filter_sets.push(filter_set);
        match filter_sets.len() {
            l if l % 0xF == 0 => debug!("Parsed {l} filter_sets."),
            _ => (),
        }
    }
    warn!("filter_set_worker exiting normally.");
    Ok(filter_sets)
}
