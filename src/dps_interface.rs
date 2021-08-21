use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DP {
    
    advertise_interval : u32,
    arp_neighbor_timeout : u32,
    description : String,
    dot1x : HashMap<String,DPDot1X>, 
    dp_id: u32,
    drop_broadcast_source_address : bool,
    drop_spoofed_faucet_mac : bool,
    group_table : bool,
    hardware : String,
    high_priority: u32,
    highest_priority: u32,
    ignore_learn_ins: u32,
    interfaces: HashMap<u32,DPInterface>,
    interface_ranges : HashMap<String,DPInterface>,
    learn_ban_timeout : u32,
    learn_jitter : u32,
    lldp_beacon : HashMap<String, DPLLDP>, 
    low_priority : u32,
    lowest_priority : u32,
    max_host_fib_retry_count : u32,
    max_hosts_per_resolve_cycle : u32,
    max_resolve_backoff_time : u32,
    metrics_rate_limit_sec : u32,
    name: String,
    ofchannel_log : String,
    packetin_pps : u32,
    slowpath_pps : u32,
    priority_offset : u32,
    proactive_learn_v4 : bool,
    proactive_learn_v6 : bool,
    stack : HashMap<String,DPStack>,
    timeout : u32,
    use_idle_timeout : bool,
    table_sizes : HashMap<String,String>, //noted in defects, unsure of attribute type
    port_table_scale_factor : f32,
    global_vlan : u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPStack {
    priority : u32,
    down_time_multiple : u32,
    min_stack_health : f32,
    min_lacp_health : f32

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPLLDP {
    system_name : String,
    send_interval : u32,
    max_per_interval : u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPDot1X{
    nfv_intf : String,
    nfv_sw_port : u32,
    radius_ip : String,
    radius_port : u32,
    radius_secret : String,
    noauth_acl : String,
    auth_acl : String,
    dot1x_assigned : bool
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPInterface {
    acl_in : String,
    #[serde(default)]
    acls_in : Vec<String>,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_str")]
    #[serde(default)]
    description: String,
    dot1x : bool,
    dot1x_acl : bool,
    dot1x_mab : bool,
    enabled : bool,
    hairpin : bool,
    lldp_beacon : HashMap<String,DPInterfaceLLDPBeacon>, 
    loop_protect : bool,
    loop_protect_external : bool,
    max_hosts : u32,
    mirror : Vec<String>,
    name: String,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_str")]
    #[serde(default)]
    native_vlan: String,
    number : u32,
    opstatus_reconf : bool,
    output_only : bool,
    permanent_learn : bool,
    stack : HashMap<String,DPInterfaceStack>,
    tagged_vlans : Vec<String>,
    unicast_flood : bool,
    restricted_bcast_arpnd : bool,
    coprocessor : HashMap<String, String>, //defect noted uncertain about this attribute type
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPInterfaceStack {
    dp : String,
    port : String

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPInterfaceLLDPBeacon {
    enable : bool,
    org_tlvs : Vec<DPInterfaceLLDPBeaconTLV>,
    port_descr : String,
    system_name : String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPInterfaceLLDPBeaconTLV {
    info : String,
    oui : u32,
    subtype : u32
}
