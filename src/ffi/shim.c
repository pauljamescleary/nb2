#include <rte_errno.h>
#include <rte_ethdev.h>
#include <rte_lcore.h>

int
_rte_errno(void) {
    return rte_errno;
}

unsigned
_rte_lcore_id(void) {
    return rte_lcore_id();
}

unsigned
_rte_lcore_to_socket_id(unsigned lcore_id) {
    return rte_lcore_to_socket_id(lcore_id);
}

uint16_t _rte_eth_rx_burst(
    uint16_t port_id,
    uint16_t queue_id,
    struct rte_mbuf **rx_pkts,
    const uint16_t nb_pkts) {
    return rte_eth_rx_burst(port_id, queue_id, rx_pkts, nb_pkts);
}

uint16_t _rte_eth_tx_burst(
    uint16_t port_id,
    uint16_t queue_id,
    struct rte_mbuf **tx_pkts,
    uint16_t nb_pkts) {
    return rte_eth_tx_burst(port_id, queue_id, tx_pkts, nb_pkts);
}
