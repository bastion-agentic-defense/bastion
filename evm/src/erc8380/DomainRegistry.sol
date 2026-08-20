// SPDX-License-Identifier: CC0-1.0
pragma solidity ^0.8.28;

/// @title DomainRegistry — Minimal Domain Lifecycle
/// @notice Tracks which orchestrator domains are active. Separate from ERC-8354's
///         policy-domain registry to preserve standard independence.
contract DomainRegistry {
    struct Domain {
        bool registered;
        bool revoked;
    }

    mapping(uint => Domain) public domains;

    event DomainRegistered(uint indexed homeDomainId);
    event DomainRevoked(uint indexed homeDomainId);

    /// @notice Register a new orchestrator domain.
    function registerDomain(
        uint homeDomainId
    ) external {
        require(!domains[homeDomainId].registered, "UAC: domain already registered");
        domains[homeDomainId] = Domain(true, false);
        emit DomainRegistered(homeDomainId);
    }

    /// @notice Revoke an existing domain.
    function revokeDomain(
        uint homeDomainId
    ) external {
        require(domains[homeDomainId].registered, "UAC: domain not registered");
        require(!domains[homeDomainId].revoked, "UAC: domain already revoked");
        domains[homeDomainId].revoked = true;
        emit DomainRevoked(homeDomainId);
    }

    /// @notice Query whether a domain is registered and not revoked.
    function isActiveDomain(
        uint homeDomainId
    ) external view returns (bool) {
        return domains[homeDomainId].registered && !domains[homeDomainId].revoked;
    }
}
