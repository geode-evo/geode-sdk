#pragma once

#include "../Platform.hpp"

#if defined(TULIP_HOOK_MACOS) && defined(TULIP_HOOK_X64)

#include "../CallingConvention.hpp"

#include <memory>
#include <string>

namespace tulip::hook {
	class AbstractFunction;

	class TULIP_HOOK_DLL SystemVConvention : public CallingConvention {
	public:
		~SystemVConvention() override;

		void generateDefaultCleanup(BaseAssembler& a, AbstractFunction const& function) override;
		void generateIntoDefault(BaseAssembler& a, AbstractFunction const& function) override;
		void generateIntoOriginal(BaseAssembler& a, AbstractFunction const& function) override;
		void generateOriginalCleanup(BaseAssembler& a, AbstractFunction const& function) override;
		bool needsWrapper(AbstractFunction const& function) const override;

		static std::shared_ptr<SystemVConvention> create();
	};
}

#endif
