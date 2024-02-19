import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import {
  AccessControl,
  AccessControlError,
} from "../../types/AccessControl.type";
import { AccessControlInternal } from "../../types/AccessControlInternal.type";
import "chai-as-promised";
import "wookashwackomytest-polkahat-chai-matchers";

export const DEFAULT_ADMIN_ROLE = 0;
export const ROLE = 1;
export const OTHER_ROLE = 2;

type ShouldBehaveLikeAccessControlParams = {
  mock: AccessControl & AccessControlInternal;
  defaultAdmin: KeyringPair;
  accounts: KeyringPair[];
};

export function shouldBehaveLikeAccessControl(
  getParams: () => ShouldBehaveLikeAccessControlParams
) {
  const ctx: ShouldBehaveLikeAccessControlParams & {
    authorized: KeyringPair;
    other: KeyringPair;
    otherAdmin: KeyringPair;
  } = {} as any;
  beforeEach(async function () {
    Object.assign(ctx, getParams());
    ctx.authorized = ctx.accounts[0];
    ctx.other = ctx.accounts[1];
    ctx.otherAdmin = ctx.accounts[2];
  });

  describe("default admin", function () {
    it("deployer has default admin role", async function () {
      expect(
        (
          await ctx.mock.query.hasRole(
            DEFAULT_ADMIN_ROLE,
            ctx.defaultAdmin.address
          )
        ).value.ok
      ).to.be.true;
    });

    it("other roles's admin is the default admin role", async function () {
      expect(
        (await ctx.mock.query.getRoleAdmin(ROLE)).value.ok?.toNumber()
      ).to.equal(DEFAULT_ADMIN_ROLE);
    });

    it("default admin role's admin is itself", async function () {
      expect(
        (
          await ctx.mock.query.getRoleAdmin(DEFAULT_ADMIN_ROLE)
        ).value.ok?.toNumber()
      ).to.equal(DEFAULT_ADMIN_ROLE);
    });
  });

  describe("granting", function () {
    beforeEach(async function () {
      await ctx.mock
        .withSigner(ctx.defaultAdmin)
        .tx.grantRole(ROLE, ctx.authorized.address);
    });

    it("non-admin cannot grant role to other accounts", async function () {
      await expect(
        ctx.mock
          .withSigner(ctx.other)
          .query.grantRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });

    it("accounts cannot be granted a role multiple times", async function () {
      await ctx.mock
        .withSigner(ctx.defaultAdmin)
        .tx.grantRole(ROLE, ctx.other.address);
      await expect(
        ctx.mock
          .withSigner(ctx.defaultAdmin)
          .query.grantRole(ROLE, ctx.other.address)
      ).to.be.revertedWithError(AccessControlError.roleRedundant);
    });
  });

  describe("revoking", function () {
    it("roles that are not had cannot be revoked", async function () {
      expect(
        (await ctx.mock.query.hasRole(ROLE, ctx.authorized.address)).value.ok
      ).to.be.false;

      await expect(
        ctx.mock
          .withSigner(ctx.defaultAdmin)
          .query.revokeRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });

    describe("with granted role", function () {
      beforeEach(async function () {
        await ctx.mock
          .withSigner(ctx.defaultAdmin)
          .tx.grantRole(ROLE, ctx.authorized.address);
      });

      it("admin can revoke role", async function () {
        await expect(
          ctx.mock
            .withSigner(ctx.defaultAdmin)
            .tx.revokeRole(ROLE, ctx.authorized.address)
        ).to.emitEvent(ctx.mock, "RoleRevoked", {
          role: ROLE,
          account: ctx.authorized.address,
          sender: ctx.defaultAdmin.address,
        });

        expect(
          (await ctx.mock.query.hasRole(ROLE, ctx.authorized.address)).value.ok
        ).to.be.false;
      });

      it("non-admin cannot revoke role", async function () {
        await expect(
          ctx.mock
            .withSigner(ctx.other)
            .query.revokeRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });

      it("a role cannot be revoked multiple times", async function () {
        await ctx.mock
          .withSigner(ctx.defaultAdmin)
          .tx.revokeRole(ROLE, ctx.authorized.address);

        await expect(
          ctx.mock
            .withSigner(ctx.defaultAdmin)
            .query.revokeRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });
    });
  });

  describe("renouncing", function () {
    it("roles that are not had cannot be renounced", async function () {
      const queryRes = await ctx.mock.query.hasRole(
        ROLE,
        ctx.authorized.address
      );
      await expect(
        ctx.mock
          .withSigner(ctx.authorized)
          .query.renounceRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });

    describe("with granted role", function () {
      beforeEach(async function () {
        await ctx.mock
          .withSigner(ctx.defaultAdmin)
          .tx.grantRole(ROLE, ctx.authorized.address);
      });

      it("bearer can renounce role", async function () {
        await expect(
          ctx.mock
            .withSigner(ctx.authorized)
            .tx.renounceRole(ROLE, ctx.authorized.address)
        ).to.emitEvent(ctx.mock, "RoleRevoked", {
          role: ROLE,
          account: ctx.authorized.address,
          sender: ctx.authorized.address,
        });

        expect(
          (await ctx.mock.query.hasRole(ROLE, ctx.authorized.address)).value?.ok
        ).to.be.false;
      });

      it("only the sender can renounce their roles", async function () {
        expect(
          ctx.mock
            .withSigner(ctx.defaultAdmin)
            .query.renounceRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.invalidCaller);
      });

      it("a role cannot be renounced multiple times", async function () {
        await ctx.mock
          .withSigner(ctx.authorized)
          .tx.renounceRole(ROLE, ctx.authorized.address);

        await expect(
          ctx.mock
            .withSigner(ctx.authorized)
            .query.renounceRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });
    });
  });

  describe("setting role admin", function () {
    beforeEach(async function () {
      await expect(
        ctx.mock.withSigner(ctx.defaultAdmin).tx.setRoleAdmin(ROLE, OTHER_ROLE)
      ).to.emitEvent(ctx.mock, "RoleAdminChanged", {
        role: ROLE,
        previous: DEFAULT_ADMIN_ROLE,
        new: OTHER_ROLE,
      });

      await ctx.mock
        .withSigner(ctx.defaultAdmin)
        .tx.grantRole(OTHER_ROLE, ctx.otherAdmin.address);
    });

    it("a role's admin role can be changed", async function () {
      expect(
        (await ctx.mock.query.getRoleAdmin(ROLE)).value.ok?.toNumber()
      ).to.equal(OTHER_ROLE);
    });

    it("the new admin can grant roles", async function () {
      await expect(
        ctx.mock
          .withSigner(ctx.otherAdmin)
          .tx.grantRole(ROLE, ctx.authorized.address)
      ).to.emitEvent(ctx.mock, "RoleGranted", {
        role: ROLE,
        grantee: ctx.authorized.address,
        grantor: ctx.otherAdmin.address,
      });
    });

    it("the new admin can revoke roles", async function () {
      await ctx.mock
        .withSigner(ctx.otherAdmin)
        .tx.grantRole(ROLE, ctx.authorized.address);
      await expect(
        ctx.mock
          .withSigner(ctx.otherAdmin)
          .tx.revokeRole(ROLE, ctx.authorized.address)
      ).to.emitEvent(ctx.mock, "RoleRevoked", {
        role: ROLE,
        account: ctx.authorized.address,
        sender: ctx.otherAdmin.address,
      });
    });

    it("a role's previous admins no longer grant roles", async function () {
      await expect(
        ctx.mock
          .withSigner(ctx.defaultAdmin)
          .query.grantRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });

    it("a role's previous admins no longer revoke roles", async function () {
      await expect(
        ctx.mock
          .withSigner(ctx.defaultAdmin)
          .query.revokeRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });
  });

  describe("internal functions", function () {
    describe("onlyRole modifier", function () {
      beforeEach(async function () {
        await ctx.mock
          .withSigner(ctx.defaultAdmin)
          .tx.grantRole(ROLE, ctx.authorized.address);
      });

      it("do not revert if sender has role", async function () {
        expect(
          (ctx.mock as AccessControlInternal)
            .withSigner(ctx.authorized)
            .query.tEnsureHasRole(ROLE)
        ).not.to.be.revertedWithError(AccessControlError.missingRole);
      });

      it("revert if sender doesn't have role #1", async function () {
        await expect(
          (ctx.mock as AccessControlInternal)
            .withSigner(ctx.other)
            .query.tEnsureHasRole(ROLE)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });

      it("revert if sender doesn't have role #2", async function () {
        await expect(
          (ctx.mock as AccessControlInternal)
            .withSigner(ctx.authorized)
            .query.tEnsureHasRole(OTHER_ROLE)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });
    });
    describe("_grantRole", function () {
      it("return true if the account does not have the role", async function () {
        await expect(
          ctx.mock.tx.tGrantRole(ROLE, ctx.authorized.address)
        ).to.emitEvent(ctx.mock, "RoleGranted", {
          role: ROLE,
          grantee: ctx.authorized.address,
          grantor: ctx.defaultAdmin.address,
        });
      });

      it("return false if the account has the role", async function () {
        await ctx.mock.tx.tGrantRole(ROLE, ctx.authorized.address);

        await expect(
          ctx.mock.query.tGrantRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.roleRedundant);
      });
    });

    describe("_revokeRole", function () {
      it("return true if the account has the role", async function () {
        await ctx.mock.tx.tGrantRole(ROLE, ctx.authorized.address);

        await expect(
          ctx.mock.tx.tRevokeRole(ROLE, ctx.authorized.address)
        ).to.emitEvent(ctx.mock, "RoleRevoked", {
          role: ROLE,
          account: ctx.authorized.address,
          sender: ctx.defaultAdmin.address,
        });
      });

      it("return false if the account does not have the role", async function () {
        await expect(
          (ctx.mock as AccessControlInternal).query.tRevokeRole(
            ROLE,
            ctx.authorized.address
          )
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });
    });
  });
}

// /**
//  * @dev At least one of the following rules was violated:
//  *
//  * - The `DEFAULT_ADMIN_ROLE` must only be managed by itself.
//  * - The `DEFAULT_ADMIN_ROLE` must only be held by one account at the time.
//  * - Any `DEFAULT_ADMIN_ROLE` transfer must be in two delayed steps.
//  */
// error AccessControlEnforcedDefaultAdminRules();

// export function shouldBehaveLikeAccessControlDefaultAdminRules(
//   getParams: () => ShouldBehaveLikeAccessControlParams
// ) {
//   const ctx: ShouldBehaveLikeAccessControlParams & {
//     authorized: KeyringPair;
//     other: KeyringPair;
//     otherAdmin: KeyringPair;
//   } = {} as any;
//   beforeEach(async function () {
//     Object.assign(ctx, getParams());
//     ctx.authorized = ctx.accounts[0];
//     ctx.other = ctx.accounts[1];
//     ctx.otherAdmin = ctx.accounts[2];
//   });

//   for (const getter of ["owner", "defaultAdmin"] as const) {
//     describe(`${getter}()`, function () {
//       it("has a default set to the initial default admin", async function () {
//         const value = await ctx.mock.query[getter]();
//         expect(value).to.equal(ctx.defaultAdmin);
//         expect(await ctx.mock.tx.hasRole(DEFAULT_ADMIN_ROLE, value)).to.be.true;
//       });

//       it("changes if the default admin changes", async function () {
//         // Starts an admin transfer
//         await ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .beginDefaultAdminTransfer(ctx.newDefaultAdmin);

//         const value = await ctx.mock[getter]();
//         expect(value).to.equal(ctx.newDefaultAdmin);
//       });
//     });
//   }

//   it("should revert if granting default admin role", async function () {
//     await expect(
//       ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .grantRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)
//     ).to.be.revertedWithError(
//       ctx.mock,
//       "AccessControlEnforcedDefaultAdminRules"
//     );
//   });

//   it("should revert if revoking default admin role", async function () {
//     await expect(
//       ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .revokeRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)
//     ).to.be.revertedWithError(
//       ctx.mock,
//       "AccessControlEnforcedDefaultAdminRules"
//     );
//   });

//   it("should revert if defaultAdmin's admin is changed", async function () {
//     await expect(
//       ctx.mock.tx.$_setRoleAdmin(DEFAULT_ADMIN_ROLE, OTHER_ROLE)
//     ).to.be.revertedWithError(
//       ctx.mock,
//       "AccessControlEnforcedDefaultAdminRules"
//     );
//   });

//   it("should not grant the default admin role twice", async function () {
//     await expect(
//       ctx.mock.tx.tGrantRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)
//     ).to.be.revertedWithError(
//       ctx.mock,
//       "AccessControlEnforcedDefaultAdminRules"
//     );
//   });

//   // tu rzeba cos wywalic
//   describe("renounces admin", function () {
//     beforeEach(async function () {
//       await ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .beginDefaultAdminTransfer(ethers.ZeroAddress);
//       ctx.expectedSchedule = (await time.clock.timestamp()) + ctx.delay;
//     });

//     it("reverts if caller is not default admin", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);
//       await expect(
//         ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .renounceRole(DEFAULT_ADMIN_ROLE, ctx.other)
//       ).to.be.revertedWithError(ctx.mock, "AccessControlBadConfirmation");
//     });

//     it("renouncing the admin role when not an admin doesn't affect the schedule", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);
//       await ctx.mock
//         .withSigner(ctx.other)
//         .renounceRole(DEFAULT_ADMIN_ROLE, ctx.other);

//       const { newAdmin, schedule } = await ctx.mock.tx.pendingDefaultAdmin();
//       expect(newAdmin).to.equal(ethers.ZeroAddress);
//       expect(schedule).to.equal(ctx.expectedSchedule);
//     });

//     it("keeps defaultAdmin consistent with hasRole if another non-defaultAdmin user renounces the DEFAULT_ADMIN_ROLE", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);

//       // This passes because it's a noop
//       await ctx.mock
//         .withSigner(ctx.other)
//         .renounceRole(DEFAULT_ADMIN_ROLE, ctx.other);

//       expect(await ctx.mock.tx.hasRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)).to
//         .be.true;
//       expect(await ctx.mock.tx.defaultAdmin()).to.equal(ctx.defaultAdmin);
//     });

//     it("renounces role", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);
//       await expect(
//         ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .renounceRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)
//       )
//         .to.emitEvent(ctx.mock, "RoleRevoked")
//         .withArgs(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin, ctx.defaultAdmin);

//       expect(await ctx.mock.tx.hasRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)).to
//         .be.false;
//       expect(await ctx.mock.tx.defaultAdmin()).to.equal(ethers.ZeroAddress);
//       expect(await ctx.mock.tx.owner()).to.equal(ethers.ZeroAddress);

//       const { newAdmin, schedule } = await ctx.mock.tx.pendingDefaultAdmin();
//       expect(newAdmin).to.equal(ethers.ZeroAddress);
//       expect(schedule).to.equal(0);
//     });

//     it("allows to recover access using the internal _grantRole", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);
//       await ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .renounceRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin);

//       await expect(
//         ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .tGrantRole(DEFAULT_ADMIN_ROLE, ctx.other)
//       )
//         .to.emitEvent(ctx.mock, "RoleGranted")
//         .withArgs(DEFAULT_ADMIN_ROLE, ctx.other, ctx.defaultAdmin);
//     });
//   });
// }
